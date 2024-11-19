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
    
    framebuffer.set_background_color(Color::new(0, 0, 0));
    
    let mut translation = Vec3::new(-40.0, 0.0, 0.0);
    let mut rotation = Vec3::new(-3.1, 0.0, 0.0);
    let mut scale = 20.0f32;
    let mut translation2 = Vec3::new(50.0, 100.0, 100.0);

    let mut camera = Camera {
        eye: Vec3::new(0.0, 0.0, 500.0),
        center: Vec3::new(translation.x - 50.0, translation.y, translation.z),
        up: Vec3::new(0.0, 1.0, 0.0), 
        has_changed: true,
    };

    let obj = Obj::load("src/ship.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 
    let mut time = 0;
    let mut time2 = 0;

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        time += 1;
        time2 += 1;

        handle_input(&window, &mut translation, &mut rotation, &mut scale, &mut camera);

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
        let noise2 = create_noise();
        let model_matrix2 = create_model_matrix(translation2, scale, rotation);
        let view_matrix2 = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix2 = create_perspective_matrix(width as f32, height as f32);
        let viewport_matrix2 = create_viewport_matrix(width as f32, height as f32);
        let uniforms2 = Uniforms { 
            model_matrix:model_matrix2, 
            view_matrix:view_matrix2,
            projection_matrix:projection_matrix2,
            viewport_matrix:viewport_matrix2,
            time,
            noise: noise2,
        };

        framebuffer.set_current_color(Color::new(0,0,0));
        render(&mut framebuffer, &uniforms, &vertex_arrays);
        //render(&mut framebuffer, &uniforms2, &vertex_arrays);

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(
    window: &Window,
    translation: &mut Vec3,
    rotation: &mut Vec3,
    scale: &mut f32,
    camera: &mut Camera,
) {
    let smooth_factor = 0.1;
    let movement_speed = 5.0;
    let rotation_speed = 0.05;
    let orbit_radius = 500.0;

    let mut move_forward = false;
    let mut move_left = false;
    let mut move_right = false;

    // Control de la cámara
    if window.is_key_down(Key::Right) {
        camera.orbit(-0.05, 0.0);
    }
    if window.is_key_down(Key::Left) {
        camera.orbit(0.05, 0.0);
    }
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0, 0.05);
    }
    if window.is_key_down(Key::Down) {
        camera.orbit(0.0, -0.05);
    }

    // Movimiento hacia adelante
    if window.is_key_down(Key::W) {
        move_forward = true;

        if rotation.x > -3.1 {
            rotation.x -= smooth_factor * 0.56;
        }
        if rotation.z.abs() > 0.0 {
            rotation.z *= 1.0 -smooth_factor;
        }
    }

    // Movimiento diagonal hacia adelante-izquierda (W + A)
    if window.is_key_down(Key::A) {
        move_left = true;
    }

    // Movimiento diagonal hacia adelante-derecha (W + D)
    if window.is_key_down(Key::D) {
        move_right = true;
    }

    if move_forward {
        let angle = rotation.y;
        let forward_x = angle.sin() * movement_speed;
        let forward_z = angle.cos() * movement_speed;

        translation.x -= forward_x;
        translation.z -= forward_z;

        camera.eye.x = translation.x + orbit_radius * angle.sin();
        camera.eye.z = translation.z + orbit_radius * angle.cos();
        camera.center.z = translation.z;
        camera.center.x = translation.x - 50.0;

        println!("Forward -> Translation: {:?}", translation);
        print!("Eye: {:?}", camera.eye);
        //println!("Rotation -> {:?}", rotation);

    }

    if move_forward && move_left {

        let adjusted_angle = rotation.y + PI / 4.0;
        let forward_x = adjusted_angle.sin() * movement_speed;
        let forward_z = adjusted_angle.cos() * movement_speed;

        translation.x -= forward_x;
        translation.z -= forward_z;
        rotation.z = lerp(rotation.z, 6.0 * (PI / 20.0), smooth_factor);
        println!("Diagonal Izquierda -> Rotation Y: {}", rotation.y);
    }

    if move_forward && move_right {
        
        let adjusted_angle = rotation.y - PI/4.0;
        let forward_x = adjusted_angle.sin() * movement_speed;
        let forward_z = adjusted_angle.cos() * movement_speed;

        translation.x -= forward_x;
        translation.z -= forward_z;

        rotation.z = lerp(rotation.z, -6.0 * (PI / 20.0), smooth_factor);
        println!("Diagonal Derecha -> Rotation Y: {}", rotation.y);
    }

    // Otros controles de rotación
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

fn update_camera_center(angle: f32, radius: f32, camera: &mut Camera, translation: Vec3) {
    camera.eye.x = translation.x + radius * angle.sin() + 200.0;
    camera.eye.z = translation.z + radius * angle.cos() + 200.0;
}


fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
