//main.rs

use nalgebra_glm::{Vec3, Vec2};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangule;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shader;
mod camera;
mod bmp;
mod render;
mod skybox;
mod minimap;
mod polygon;
mod line;

use minimap::Minimap;
use skybox::Skybox;
use color::Color;
use framebuffer::Framebuffer;
use obj::Obj;
use camera::Camera;
use crate::render::{Uniforms, render, create_model_matrix, create_view_matrix, create_perspective_matrix, create_viewport_matrix};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

fn create_noise(number: u8) -> FastNoiseLite {
    if number == 0 {
        create_earth_noise()
    } else if number == 1 {
        create_earth_noise()
    } else if number == 2 {
        create_magma_noise()
    } else if number == 3 {
        create_ice_noise()
    } else if number == 4 {
        create_star_noise()
    } else if number == 5 {
        create_ground_noise()
    } else {
        create_earth_noise()
    }
}

fn create_earth_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1345);

    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_frequency(Some(0.005));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(4));
    noise.set_fractal_gain(Some(0.5));
    noise.set_fractal_lacunarity(Some(2.0));

    noise
}

fn create_magma_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(51);

    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_fractal_type(Some(FractalType::FBm));
    noise.set_fractal_octaves(Some(5));
    noise.set_fractal_lacunarity(Some(2.5));
    noise.set_fractal_gain(Some(0.5));
    noise.set_frequency(Some(0.002));

    noise
}

fn create_ice_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    noise.set_noise_type(Some(NoiseType::OpenSimplex2));
    noise.set_fractal_type(Some(FractalType::Ridged));
    noise.set_frequency(Some(2.5));
    noise
}

fn create_star_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1254);
    noise.set_noise_type(Some(NoiseType::Perlin));
    noise.set_frequency(Some(0.1));
    noise
}

fn create_ground_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    
    // Use FBm fractal type to layer multiple octaves of noise
    noise.set_noise_type(Some(NoiseType::Cellular)); // Cellular noise for cracks
    noise.set_fractal_type(Some(FractalType::FBm));  // Fractal Brownian Motion
    noise.set_fractal_octaves(Some(5));              // More octaves = more detail
    noise.set_fractal_lacunarity(Some(2.0));         // Lacunarity controls frequency scaling
    noise.set_fractal_gain(Some(0.5));               // Gain controls amplitude scaling
    noise.set_frequency(Some(0.05));                 // Lower frequency for larger features

    noise
}

fn main() {
    let width = 900;
    let height = 800;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(width, height);
    let mut window = Window::new(
        "Rust Graphics - Renderer Example",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap();

    window.update();
    
    framebuffer.set_background_color(Color::new(0, 0, 0));
    
    let mut translation = Vec3::new(-7000.0, 0.0, 0.0);
    let mut rotation_y = 0.0; // Cambiado a un único valor para la rotación Y
    let mut rotation_x = -3.1;
    let mut rotation_z = 0.0;
    let mut scale = 20.0f32;

    let mut camera = Camera {
        eye: Vec3::new(0.0, 0.0, translation.z + 500.0),
        center: Vec3::new(translation.x - 50.0, translation.y, translation.z),
        up: Vec3::new(0.0, 1.0, 0.0), 
        has_changed: true,
    };

    let obj = Obj::load("src/ship.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 
    
    let sphere = Obj::load("src/sphere.obj").expect("Failed to load obj");
    let vertex_arrays_sphere = sphere.get_vertex_array();

    let mut time = 0;

    let skybox = Skybox::new(10000);

    let mut celestial_bodies = vec![
        (vertex_arrays_sphere.clone(), Vec3::new(0.0, 0.0, 0.0), 3000.0, 4, 0.0), //star
        (vertex_arrays_sphere.clone(), Vec3::new(2000.0 / 2.0f32.sqrt(), 0.0, 2000.0 / 2.0f32.sqrt()), 3000.0, 2, 0.0),
        (vertex_arrays_sphere.clone(), Vec3::new(4000.0 / 2.0f32.sqrt(), 0.0, -4000.0 / 2.0f32.sqrt()), 2500.0, 3, 1.57),
        (vertex_arrays_sphere.clone(), Vec3::new(-6000.0 / 2.0f32.sqrt(), 0.0, -6000.0 / 2.0f32.sqrt()), 1800.0, 1, 3.14),
        (vertex_arrays_sphere.clone(), Vec3::new(-8000.0 / 2.0f32.sqrt(), 0.0, 8000.0 / 2.0f32.sqrt()), 1800.0, 5, 4.71),
        (vertex_arrays_sphere.clone(), Vec3::new(-8000.0, 0.0, 0.0), 1800.0, 6, 6.28),
    ];

    let mut minimap = Minimap::new(
        (width as isize - 100) / 4, 
        height as isize / 4, 
        Vec2::new((width - 120) as f32, 120.0), 
        Vec2::new(translation.x, translation.z),
        Vec2::new(celestial_bodies[1].1.x, celestial_bodies[1].1.z),
        Vec2::new(celestial_bodies[2].1.x, celestial_bodies[2].1.z),
        Vec2::new(celestial_bodies[3].1.x, celestial_bodies[3].1.z),
        Vec2::new(celestial_bodies[4].1.x, celestial_bodies[4].1.z),
        Vec2::new(celestial_bodies[5].1.x, celestial_bodies[5].1.z),
    );

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;

        // Manejo de entrada actualizado con desacoplamiento
        handle_input(
            &window, 
            &mut translation, 
            &mut rotation_y, 
            &mut rotation_x,
            &mut rotation_z,
            &mut scale, 
            &mut camera, 
            &mut minimap,
        );

        framebuffer.clear();

        let noise = create_noise(0);
        let model_matrix = create_model_matrix(translation, scale, Vec3::new(rotation_x, rotation_y, rotation_z));
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(width as f32, height as f32);
        let viewport_matrix = create_viewport_matrix(width as f32, height as f32);
        let uniforms_base = Uniforms { 
            model_matrix, 
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise,
        };

        skybox.render(&mut framebuffer, &uniforms_base, camera.eye);

        render(&mut framebuffer, &uniforms_base, &vertex_arrays, 0);
        
        for (vertex_array, translation, scale, number, _) in &celestial_bodies {
             if is_in_center(*translation, &mut camera) {

                let distance = (camera.eye - *translation).magnitude();

                let new_scale = calculate_scale_based_on_distance(distance, *scale);
                let noise = create_noise(*number);
                let model_matrix = create_model_matrix(*translation, new_scale, Vec3::zeros());
                let uniforms = Uniforms {
                    model_matrix,
                    noise,
                    ..uniforms_base
                };
                render(&mut framebuffer, &uniforms, vertex_array, *number);
            }
        }

        for body in &mut celestial_bodies {
            let (_, position, _, number, angle) = body;

            *angle += 0.001 as f32;

            let radius = position.magnitude();
            position.x = radius * angle.cos();
            position.z = radius * angle.sin();

            if *number == 1 {
                minimap.update_p1_pos(position.x, position.z);
            } else if *number == 2 {
                minimap.update_p2_pos(position.x, position.z);
            } else if *number == 3 {
                minimap.update_p3_pos(position.x, position.z);
            } else if *number == 4 {

            } else if *number == 5 {
                minimap.update_p4_pos(position.x, position.z);
            } else {
                minimap.update_p5_pos(position.x, position.z);
            }
        }

        minimap.render(&mut framebuffer);

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn calculate_scale_based_on_distance(distance: f32, max_scale: f32) -> f32 {
    // Definir un mínimo y un máximo de escala
    let min_scale = 5.0;

    let max_distance = 10000.0; // La distancia máxima para la escala
    let scale_factor = (distance / max_distance).clamp(0.0, 1.0);
    min_scale + (max_scale - min_scale) * (1.0 - scale_factor) // A mayor distancia, menor escala
}

fn is_in_center(translation: Vec3, camera: &mut Camera) -> bool {
    let camera_xy = Vec2::new(camera.eye.x, camera.eye.y);
    let translation_xy = Vec2::new(translation.x, translation.y);
    let range = 3000.0; // Rango permitido en unidades.

    // Verificar si está dentro del rango en ambas dimensiones.
    let in_range = (translation_xy.x >= camera_xy.x - range && translation_xy.x <= camera_xy.x + range) &&
                   (translation_xy.y >= camera_xy.y - range && translation_xy.y <= camera_xy.y + range);

    if !in_range {
        return false;
    }

    // Vector dirección desde la cámara hacia el objeto.
    let direction_to_object = translation - camera.eye;

    // Vector dirección de la cámara.
    let camera_direction = camera.center - camera.eye;

    // Normalizar los vectores para evitar problemas de escala.
    let direction_to_object_norm = direction_to_object.normalize();
    let camera_direction_norm = camera_direction.normalize();

    // Calcular el producto punto para obtener el coseno del ángulo.
    let dot_product = direction_to_object_norm.dot(&camera_direction_norm);

    // El ángulo máximo permitido (ajustable según el FOV de la cámara).
    let fov_cosine = (45.0f32.to_radians()).cos(); // 45 grados de FOV como ejemplo.

    // Verificar si el objeto está dentro del FOV.
    dot_product > fov_cosine
}

fn handle_input(
    window: &Window,
    translation: &mut Vec3,
    rotation_y: &mut f32, // Solo el ángulo Y es necesario.
    rotation_x: &mut f32,
    rotation_z: &mut f32,
    scale: &mut f32,
    camera: &mut Camera,
    minimap: &mut Minimap,
) {
    let smooth_factor = 0.1;
    let movement_speed = 50.0;
    let rotation_speed = 0.05;
    let orbit_radius = 500.0;

    let mut move_direction = Vec3::new(0.0, 0.0, 0.0);
    let mut orientation = Vec3::new(rotation_y.sin(), 0.0, rotation_y.cos());

    // Rotación de la cámara/orientación
    if window.is_key_down(Key::Right) {
        camera.orbit(-0.05, 0.0);
    }
    if window.is_key_down(Key::Left) {
        camera.orbit(0.05, 0.0);
    }

    // Movimiento en la dirección de la orientación
    if window.is_key_down(Key::W) {
        move_direction += orientation * -movement_speed;

        camera.eye = *translation + orbit_radius * orientation;
        camera.center = *translation;

        *rotation_x = lerp(*rotation_x, -3.1, smooth_factor);
    }

    // Movimiento lateral (izquierda/derecha, ortogonal a la orientación)
    let right = Vec3::new(-orientation.z, 0.0, orientation.x); // Vector perpendicular a la orientación.
    if window.is_key_down(Key::A) {
        move_direction += right * (movement_speed);
        *rotation_y += rotation_speed;
        orientation.x = rotation_y.sin();
        orientation.z = rotation_y.cos();
    }
    if window.is_key_down(Key::D) {
        move_direction -= right * movement_speed;
        *rotation_y -= rotation_speed;
        orientation.x = rotation_y.sin();
        orientation.z = rotation_y.cos();
    }

    // Movimiento vertical
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0,0.05);
    } 

    if window.is_key_down(Key::Down) {
        camera.orbit(0.0,-0.05);
    }

    if window.is_key_down(Key::Up) && window.is_key_down(Key::W) {
        move_direction.y += movement_speed;
        *rotation_x = lerp(*rotation_x, -(PI/10.0) - PI * 1.4, smooth_factor);
    }

    if window.is_key_down(Key::Down) && window.is_key_down(Key::W) {
        move_direction.y -= movement_speed;
        *rotation_x = lerp(*rotation_x, -5.0 * (PI / 10.0), smooth_factor);
    }

    // Aplicar movimiento y actualizar posición de la cámara
    *translation += move_direction;

    // Minimapa actualizado
    minimap.update_ship_pos(translation.x, translation.z);
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

