use nalgebra_glm::{Vec3, Vec4, normalize};
use rand::prelude::*;
use std::f32::consts::PI;
use crate::{Framebuffer, Uniforms, Color};

pub struct Star {
    position: Vec3,
    brightness: f32,
    size: u8,
}

pub struct Skybox {
    stars: Vec<Star>,
}

impl Skybox {
    /// Crea un nuevo Skybox con una cantidad de estrellas uniformemente distribuidas y propiedades como brillo y tamaño.
    pub fn new(star_count: usize) -> Self {
        let mut rng = thread_rng();
        let mut stars = Vec::with_capacity(star_count);

        for _ in 0..star_count {
            // Distribución uniforme en la esfera.
            let theta = rng.gen_range(0.0..2.0 * PI); // Ángulo azimutal
            let phi = rng.gen_range(0.0..PI);         // Ángulo polar

            let x = phi.sin() * theta.cos();
            let y = phi.sin() * theta.sin();
            let z = phi.cos();

            let position = normalize(&Vec3::new(x, y, z)) * 100.0; // Escala para simular un radio
            let brightness = rng.gen_range(0.5..1.0);              // Brillo entre 0.5 y 1.0
            let size: u8 = rng.gen_range(1..=3);                   // Tamaño aleatorio

            stars.push(Star { position, brightness, size });
        }

        Skybox { stars }
    }

    /// Renderiza las estrellas en el framebuffer teniendo en cuenta la posición de la cámara y la profundidad Z.
pub fn render(&self, framebuffer: &mut Framebuffer, uniforms: &Uniforms, camera_position: Vec3) {
    for star in &self.stars {
        // Calcular la posición de la estrella relativa a la cámara.
        let position = star.position + camera_position;

        // Convertir la posición a un vec4 para las transformaciones.
        let pos_vec4 = Vec4::new(position.x, position.y, position.z, 1.0);

        // Aplicar la matriz de vista y proyección.
        let projected = uniforms.projection_matrix * uniforms.view_matrix * pos_vec4;

        // Verificar si la estrella está detrás de la cámara.
        if projected.w <= 0.0 {
            continue;
        }

        // Normalizar a coordenadas NDC.
        let ndc = projected / projected.w;

        let screen_pos = uniforms.view_matrix * Vec4::new(ndc.x,ndc.y,ndc.z,1.0);

        // Convertir las coordenadas NDC a coordenadas de pantalla.
        let screen_x = screen_pos.x as i32 + ((ndc.x + 1.0) * 0.5 * framebuffer.width as f32) as i32;
        let screen_y = screen_pos.y as i32 + ((1.0 - ndc.y) * 0.5 * framebuffer.height as f32) as i32;

        // Verificar si la estrella está dentro de la pantalla.
        if screen_x >= 0
            && screen_y >= 0
            && screen_x < framebuffer.width as i32
            && screen_y < framebuffer.height as i32
        {
            // Calcular intensidad del color basado en el brillo de la estrella.
            let intensity = (star.brightness * 255.0) as u8;
            let color = Color::new(intensity as i32, intensity as i32, intensity as i32);

            // Establecer el color actual en el framebuffer.
            framebuffer.set_current_color(color);

            // Dibujar la estrella según su tamaño.
            match star.size {
                1 => framebuffer.point(screen_x as isize, screen_y as isize, 1000.0),
                2 => {
                    framebuffer.point(screen_x as isize, screen_y as isize, 1000.0);
                    framebuffer.point((screen_x + 1) as isize, screen_y as isize, 1000.0);
                    framebuffer.point(screen_x as isize, (screen_y + 1) as isize, 1000.0);
                    framebuffer.point((screen_x + 1) as isize, (screen_y + 1) as isize, 1000.0);
                }
                3 => {
                    framebuffer.point(screen_x as isize, screen_y as isize, 1000.0);
                    framebuffer.point((screen_x - 1) as isize, screen_y as isize, 1000.0);
                    framebuffer.point((screen_x + 1) as isize, screen_y as isize, 1000.0);
                    framebuffer.point(screen_x as isize, (screen_y - 1) as isize, 1000.0);
                    framebuffer.point(screen_x as isize, (screen_y + 1) as isize, 1000.0);
                }
                _ => {}
            }
        }
    }
}

}


