//main.rs

use nalgebra_glm::Vec3;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod vertex_shader;
mod camera;
mod bmp;
mod render;

use color::Color;
use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use line::triangle;
use vertex_shader::vertex_shader;
use camera::Camera;
use crate::render::{Uniforms, render, create_model_matrix, create_view_matrix, create_perspective_matrix, create_viewport_matrix};

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
    let mut scale = 20.0f32;
    
    let mut camera = Camera {
        eye: Vec3::new(15.0, 15.0, 15.0),
        center: translation,
        up: Vec3::new(0.0, 1.0, 0.0), 
        has_changed: true,
    };

    let obj = Obj::load("src/ship.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut translation, &mut rotation, &mut scale);

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(camera.eye, camera.center, camera.up);
        let projection_matrix = create_perspective_matrix(width as f32, height as f32);
        let viewport_matrix = create_viewport_matrix(width as f32, height as f32);
        let uniforms = Uniforms { 
            model_matrix, 
            view_matrix,
            projection_matrix,
            viewport_matrix,
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
