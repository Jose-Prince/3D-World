//main.rs

use nalgebra_glm::Vec3;
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

use color::Color;
use framebuffer::Framebuffer;
use obj::Obj;
use camera::Camera;
use crate::render::{Uniforms, render, create_model_matrix, create_view_matrix, create_perspective_matrix, create_viewport_matrix};
use fastnoise_lite::{FastNoiseLite, NoiseType, FractalType};

fn create_noise() -> FastNoiseLite {
    create_earth_noise()
    //create_magma_noise()
    //create_ice_noise()
    //create_cloud_noise()
    //create_lava_noise()
    //create_ground_noise()
    //create_cell_noise()
    //create_star_noise()
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

fn create_cloud_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Cellular));
    noise
}

fn create_cell_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(1337);
    noise.set_noise_type(Some(NoiseType::Cellular));
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

fn create_lava_noise() -> FastNoiseLite {
    let mut noise = FastNoiseLite::with_seed(42);
    
    // Use FBm for multi-layered noise, giving a "turbulent" feel
    noise.set_noise_type(Some(NoiseType::Perlin));  // Perlin noise for smooth, natural texture
    noise.set_fractal_type(Some(FractalType::FBm)); // FBm for layered detail
    noise.set_fractal_octaves(Some(6));             // High octaves for rich detail
    noise.set_fractal_lacunarity(Some(2.0));        // Higher lacunarity = more contrast between layers
    noise.set_fractal_gain(Some(0.5));              // Higher gain = more influence of smaller details
    noise.set_frequency(Some(0.002));                // Low frequency = large features
    
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
    
    framebuffer.set_background_color(Color::new(255, 255, 221));
    
    let mut translation = Vec3::new(700.0, 500.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 500.0f32;
    
    let mut camera = Camera {
        eye: Vec3::new(15.0, 15.0, 15.0),
        center: translation,
        up: Vec3::new(0.0, 1.0, 0.0), 
        has_changed: true,
    };

    let obj = Obj::load("src/sphere.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 
    let mut time = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;

        handle_input(&window, &mut translation, &mut rotation, &mut scale);

        framebuffer.clear();

        let noise = create_noise();
        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(width as f32, height as f32);
        let viewport_matrix = create_viewport_matrix(width as f32, height as f32);
        let uniforms = Uniforms { 
            model_matrix, 
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
            noise,
        };

        framebuffer.set_current_color(Color::new(0,0,0));
        render(&mut framebuffer, &uniforms, &vertex_arrays);

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32) {
    if window.is_key_down(Key::Right) {
        translation.x -= 5.0;
    }
    if window.is_key_down(Key::Left) {
        translation.x += 5.0;
    }
    if window.is_key_down(Key::Up) {
        translation.y += 5.0;
    }
    if window.is_key_down(Key::Down) {
        translation.y -= 5.0;
    }
    if window.is_key_down(Key::S) {
        *scale += 2.0;
    }
    if window.is_key_down(Key::A) {
        *scale -= 2.0;
    }
    if window.is_key_down(Key::Q) {
        rotation.x -= PI / 20.0;
    }
    if window.is_key_down(Key::W) {
        rotation.x += PI / 20.0;
    }
    if window.is_key_down(Key::E) {
        rotation.y -= PI / 20.0;
    }
    if window.is_key_down(Key::R) {
        rotation.y += PI / 20.0;
    }
    if window.is_key_down(Key::T) {
        rotation.z -= PI / 20.0;
    }
    if window.is_key_down(Key::Y) {
        rotation.z += PI / 20.0;
    }
}
