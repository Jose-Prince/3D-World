// framebuffer.rs

use crate::bmp::write_bmp_file;
use crate::color::Color;
use std::f32::consts::PI;

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
}
