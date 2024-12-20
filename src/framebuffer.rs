// framebuffer.rs

use crate::color::Color;
use image::{GenericImageView, Pixel, Rgba};
use image::imageops::FilterType;
use rusttype::{Font, Scale, point, PositionedGlyph};

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>, // Para el color de cada píxel
    pub zbuffer: Vec<f32>, // Para la profundidad de cada píxel
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    // Constructor del framebuffer con z-buffer
    pub fn new(width: usize, height: usize) -> Self {
        let background_color = Color::new(0, 0, 0);
        let current_color = Color::new(255, 255, 255);

        let buffer = vec![background_color.to_hex(); width * height];
        let zbuffer = vec![f32::INFINITY; width * height]; // Inicializa z-buffer con valores infinitos

        Framebuffer {
            width,
            height,
            buffer,
            zbuffer,
            background_color,
            current_color,
        }
    }

    // Limpiar el framebuffer y el z-buffer
    pub fn clear(&mut self) {
        let color_hex = self.background_color.to_hex();
        self.buffer.fill(color_hex);
        self.zbuffer.fill(f32::INFINITY); // Resetea el z-buffer con valores infinitos
    }

    // Dibujar un punto con verificación del z-buffer
    pub fn point(&mut self, x: isize, y: isize, z: f32) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (self.width * y as usize) + x as usize;
            
            // Verificar si el punto está más cerca que el que ya está en el z-buffer
            if z < self.zbuffer[index] {
                self.zbuffer[index] = z; // Actualiza el z-buffer con la nueva profundidad
                self.buffer[index] = self.current_color.to_hex(); // Dibuja el píxel solo si es más cercano
            }
        }
    }

    // Cambiar el color de fondo del framebuffer
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    // Cambiar el color actual para dibujar
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    // Obtener el color de un punto en las coordenadas especificadas
    pub fn get_point(&self, x: isize, y: isize) -> Option<u32> {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (self.width * y as usize) + x as usize;
            Some(self.buffer[index])
        } else {
            None
        }
    }

    pub fn fill_polygon(&mut self, vertices: &Vec<[isize; 2]>, fill_color: Color) {
        let min_y = vertices.iter().map(|v| v[1]).min().unwrap_or(0);
        let max_y = vertices.iter().map(|v| v[1]).max().unwrap_or(0);

        for y in min_y..=max_y {
            let mut nodes = vec![];
            let mut j = vertices.len() - 1;
            for i in 0..vertices.len() {
                let vi = vertices[i];
                let vj = vertices[j];
                if vi[1] < y && vj[1] >= y || vj[1] < y && vi[1] >= y {
                    let intersect_x = vi[0] + (y - vi[1]) * (vj[0] - vi[0]) / (vj[1] - vi[1]);
                    nodes.push(intersect_x);
                }
                j = i;
            }
            nodes.sort();
            for n in (0..nodes.len()).step_by(2) {
                if n + 1 < nodes.len() {
                    for x in nodes[n]..=nodes[n + 1] {
                        self.set_current_color(fill_color);
                        self.point(x, y, 0.0);
                    }
                }
            }
        }
    }

    pub fn draw_text(&mut self, x: usize, y: usize, text: &str, color: Color, scale: f32) {
        // Cargar la fuente desde el archivo
        let font_data = include_bytes!("../fonts/mai10.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).unwrap();
    
        // Definir la escala (tamaño) del texto
        let scale = Scale::uniform(scale); // Ajusta el tamaño según sea necesario
    
        // Establecer el color actual del framebuffer
        self.set_current_color(color);
    
        // Obtener métricas verticales de la fuente
        let v_metrics = font.v_metrics(scale);
    
        // Crear una posición de punto para el layout
        let start_point = point(x as f32, y as f32 + v_metrics.ascent);
    
        // Generar los glifos posicionados para el texto dado
        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, start_point).collect();
    
        // Dibujar cada glifo en el framebuffer
        for glyph in glyphs {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                glyph.draw(|gx, gy, gv| {
                    let px = gx as i32 + bounding_box.min.x;
                    let py = gy as i32 + bounding_box.min.y;
    
                    // Verificar que el valor del glifo sea suficientemente alto para dibujar
                    if gv > 0.1 { // Ajusta el umbral según sea necesario
                        self.point(px as isize, py as isize, 0.0);
                    }
                });
            }
        }
    }

    pub fn draw_image(&mut self, image_path: &str, width: usize, height: usize) {
        // Cargar la imagen
        let img = image::open(image_path).expect("No se pudo cargar la imagen");
        let img_width = img.width() as usize;
        let img_height = img.height() as usize;
    
        // Calcular el factor de escala para ajustar la imagen al ancho del framebuffer
        let scale_factor = width as f32 / img_width as f32;
    
        // Mantener la relación de aspecto y calcular la nueva altura
        let new_height = (img_height as f32 * scale_factor) as usize;
    
        // Redimensionar la imagen manteniendo la relación de aspecto
        let scaled_img = img.resize_exact(width as u32, new_height as u32, FilterType::Lanczos3);
    
        // Crear un buffer para el framebuffer
        let mut buffer: Vec<u32> = vec![0; width * height];
    
        // Calcular el offset vertical para centrar la imagen en el framebuffer
        let vertical_offset = (height.saturating_sub(new_height)) / 2;
    
        // Dibujar la imagen redimensionada en el framebuffer, centrada verticalmente
        for y in 0..new_height.min(height) {
            for x in 0..width {
                let pixel = scaled_img.get_pixel(x as u32, y as u32);
                let rgba = pixel.0;
                let r = rgba[0] as u32;
                let g = rgba[1] as u32;
                let b = rgba[2] as u32;
                let a = rgba[3] as u32;
    
                // Insertar el píxel en el buffer con el offset vertical
                self.buffer[((y + vertical_offset) * width + x) as usize] = (a << 24) | (r << 16) | (g << 8) | b;
            }
        }
    }
}
