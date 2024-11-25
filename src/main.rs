//main.rs

use nalgebra_glm::{Vec3, Vec2};
use minifb::{Key, Window, WindowOptions};
use std::time::{Duration, Instant};
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
mod barrelRoll;
mod colisionWarning;
mod autopilot;
mod audioPlayer;

use audioPlayer::AudioPlayer;
use autopilot::Autopilot;
use colisionWarning::ColisionWarning;
use barrelRoll::BarrelRoll;
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
        eye: Vec3::new(translation.x - 50.0, translation.y, translation.z + 500.0),
        center: Vec3::new(translation.x - 50.0,translation.y, translation.z),
        up: Vec3::new(0.0, 1.0, 0.0), 
        has_changed: true,
    };

    //Setting if in gameplay or not

    let obj = Obj::load("objs/ship.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array(); 
    
    let sphere = Obj::load("objs/sphere.obj").expect("Failed to load obj");
    let vertex_arrays_sphere = sphere.get_vertex_array();

    let mut time = 0;
    let mut is_alternate_render = false;

    let skybox = Skybox::new(10000);

    let blink_interval = Duration::from_millis(1500);
    let text_blink_interval = Duration::from_millis(200);
    let mut last_blink_time = Instant::now();
    let mut show_warning = false;
    let mut show_autopilot = false;
    let mut autopilot = Autopilot::new();

    //OST
    let mut begin_screen_ost = AudioPlayer::new("audios/Super Mario Galaxy Soundtrack - Title Screen.mp3",0.5); 
    let mut gameplay_ost = AudioPlayer::new("audios/Ambient Space Synth Music (For Videos) - Adrift by Hayden Folker.mp3",0.5);

    gameplay_ost.stop();
    

    //Celestial bodies for rendering
    let mut celestial_bodies = vec![
        (vertex_arrays_sphere.clone(), Vec3::new(0.0, 0.0, 0.0), 3000.0, 4, 0.0), //star
        (vertex_arrays_sphere.clone(), Vec3::new(2.0 * 4000.0 / 2.0f32.sqrt(), 0.0, 2.0 * 4000.0 / 2.0f32.sqrt()), 3000.0, 2, 0.0),
        (vertex_arrays_sphere.clone(), Vec3::new(2.0 *8000.0 / 2.0f32.sqrt(), 0.0,2.0 * -8000.0 / 2.0f32.sqrt()), 2500.0, 3, 1.57),
        (vertex_arrays_sphere.clone(), Vec3::new(2.0 * -12000.0 / 2.0f32.sqrt(), 0.0,2.0 * -12000.0 / 2.0f32.sqrt()), 1800.0, 1, 3.14),
        (vertex_arrays_sphere.clone(), Vec3::new(2.0 * -16000.0 / 2.0f32.sqrt(), 0.0,2.0 * 16000.0 / 2.0f32.sqrt()), 1800.0, 5, 4.71),
        (vertex_arrays_sphere.clone(), Vec3::new(2.0 * -16000.0, 0.0, 0.0), 1800.0, 6, 6.28),
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

    let mut barrel_roll = BarrelRoll { active: false, progress: 0.0, rotation_y: rotation_y };
    let mut warning_message = ColisionWarning::new("DANGER!".to_string(), "Inminent gravitational field".to_string(), Color::new(255,255,255), Color::new(255,0,0));
    let mut autopilot_message = ColisionWarning::new("Autopilot Activated".to_string(), "Avoiding gravitational field".to_string(), Color::new(255,255,255), Color::new(254,138,24));

    let begin_page = "src/ship_img.jpg";
    let mut show_text = true;
    let mut enter_pressed = false;

    begin_screen_ost.play();

    while window.is_open() && !enter_pressed && !window.is_key_down(minifb::Key::Escape) {
        framebuffer.clear();
        framebuffer.draw_image(&begin_page, width, height);

        framebuffer.draw_text(width / 4, height - 750, "No UVG's Sky", Color::new(12,140,57), 100.0);

        if last_blink_time.elapsed() >= text_blink_interval {
            show_text = !show_text;
            last_blink_time = Instant::now();
        }
        
        if show_text {
            framebuffer.draw_text(width / 5, (4 * height) / 5 - 25, "Press ENTER to start game", Color::new(255,255,255), 70.0);
        }

        if window.is_key_down(minifb::Key::Enter) {
            enter_pressed = true;
            begin_screen_ost.stop();
            gameplay_ost.play();
        }

        window.update_with_buffer(&framebuffer.buffer, width, height).unwrap();
        std::thread::sleep(Duration::from_millis(16));
    }

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::M) {
            is_alternate_render = !is_alternate_render;
            std::thread::sleep(Duration::from_millis(200));
        } 

        time += 1;

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

        
        for (vertex_array, traslation, scale, number, _) in &celestial_bodies {
             if is_in_center(*traslation, &mut camera, true) && !barrel_roll.active {

                let distance = (camera.eye - *traslation).magnitude();
                if distance < 3500.0 && last_blink_time.elapsed() >= blink_interval && !show_autopilot  {
                    show_autopilot = false;
                    show_warning = !show_warning;
                }

                if distance < 2900.0 {
                    show_warning = false;
                    show_autopilot = true;
                }
                if show_warning {
                    warning_message.render(&mut framebuffer);
                }
                if show_autopilot {
                    autopilot_message.render(&mut framebuffer);
                    if !autopilot.active {
                        autopilot.start();
                    }
                }

                let noise = create_noise(*number);
                let model_matrix = create_model_matrix(*traslation, *scale, Vec3::zeros());
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

        if autopilot.active {
            if autopilot.progress < 0.5 {
                autopilot.simulated_keys = vec![Key::A];
                autopilot.progress += 0.01;
                println!("Progress: {:?}", autopilot.progress)
            } else if autopilot.distance_traveled < 1000.0 {
                autopilot.simulated_keys = vec![Key::W];
                autopilot.distance_traveled += 10.0;
            } else {
                autopilot.stop();
                show_autopilot = false;
            }
        }

        if !is_alternate_render {

            // Manejo de entrada actualizado con desacoplamiento
            if window.is_key_down(Key::W) && !autopilot.active {
                handle_input(
                    &window, 
                    &mut translation, 
                    &mut rotation_y, 
                    &mut rotation_x,
                    &mut rotation_z,
                    &mut camera, 
                    &mut minimap,
                    &mut barrel_roll,
                );
            }
            
            if autopilot.active {
                handle_autopilot(
                    &mut translation, 
                    &mut rotation_y, 
                    &mut rotation_x,
                    &mut camera, 
                    &mut minimap,
                    autopilot.simulated_keys.clone(),
                );
            }
            

            handle_camera(
                &window,
                &mut camera,
            );

            render(&mut framebuffer, &uniforms_base, &vertex_arrays, 0);

            minimap.render(&mut framebuffer);
        } else {
            camera.eye = Vec3::new(-0.00038838302, 88555.33, 8885.168);
        }

        window
            .update_with_buffer(&framebuffer.buffer, width, height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn is_in_center(translation: Vec3, camera: &mut Camera, is_up: bool) -> bool {

    if is_up {
        let camera_xz = Vec2::new(camera.eye.x, camera.eye.z);
        let translation_xz = Vec2::new(translation.x, translation.z);
        let range = 10000000.0; // Rango permitido en unidades.

        // Verificar si está dentro del rango en ambas dimensiones.
        let in_range = (translation_xz.x >= camera_xz.x - range && translation_xz.x <= camera_xz.x + range) &&
        (translation_xz.y >= camera_xz.y - range && translation_xz.y <= camera_xz.y + range);

        if !in_range {
            return false;
        }
    
    } else {
        let camera_xy = Vec2::new(camera.eye.x, camera.eye.y);
        let translation_xy = Vec2::new(translation.x, translation.y);
        let range = 3000.0; // Rango permitido en unidades.

        // Verificar si está dentro del rango en ambas dimensiones.
        let in_range = (translation_xy.x >= camera_xy.x - range && translation_xy.x <= camera_xy.x + range) &&
        (translation_xy.y >= camera_xy.y - range && translation_xy.y <= camera_xy.y + range);

        if !in_range {
            return false;
        }
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
    rotation_y: &mut f32,
    rotation_x: &mut f32,
    rotation_z: &mut f32,
    camera: &mut Camera,
    minimap: &mut Minimap,
    barrel_roll: &mut BarrelRoll,
) {
    let smooth_factor = 0.1;
    let movement_speed = 50.0;
    let rotation_speed = 0.05;
    let orbit_radius = 500.0;

    let mut move_direction = Vec3::new(0.0, 0.0, 0.0);

    let mut performing_action = false;

        // Orientación basada en la rotación actual
    let orientation = Vec3::new(rotation_y.sin(), 0.0, rotation_y.cos());
    let right = Vec3::new(-orientation.z, 0.0, orientation.x); // Vector ortogonal

    if !performing_action {
        // Movimiento hacia adelante y atrás
        if window.is_key_down(Key::W) {
            move_direction += orientation * -movement_speed;
            *rotation_x = lerp(*rotation_x, -3.1, smooth_factor);
        }
        if window.is_key_down(Key::S) {
            move_direction -= orientation * -movement_speed;
        }

        // Movimiento lateral
        if window.is_key_down(Key::A) && window.is_key_down(Key::W) {
            *rotation_y += rotation_speed;
            move_direction += right * movement_speed;
        }
        if window.is_key_down(Key::D) && window.is_key_down(Key::W) {
            *rotation_y -= rotation_speed;
            move_direction -= right * movement_speed;
        }

        // Rotación vertical (Pitch)
        if window.is_key_down(Key::Down) && window.is_key_down(Key::W) {
            move_direction.y += movement_speed;
            *rotation_x = lerp(*rotation_x, -5.0 * (PI / 10.0), smooth_factor);
        }
        if window.is_key_down(Key::Up) && window.is_key_down(Key::W) { 
             move_direction.y -= movement_speed;
            *rotation_x = lerp(*rotation_x, -(PI/10.0) - PI * 1.4, smooth_factor);
        }
    }

    //DO A BARREL ROLL
    if window.is_key_down(Key::X) && !barrel_roll.active {
        barrel_roll.active = true;
        barrel_roll.progress = 0.0;
        barrel_roll.rotation_y = *rotation_y;
        performing_action = true;
    }

    if barrel_roll.active {
        let animation_speed = 0.08;
        *rotation_y = 0.0;
        barrel_roll.progress += animation_speed;

        *rotation_z = lerp(0.0, 2.0 * PI, barrel_roll.progress);

        if barrel_roll.progress >= 1.0 {
            barrel_roll.active = false;
            barrel_roll.progress = 0.0;
            *rotation_z = 0.0;
            *rotation_y = barrel_roll.rotation_y;
            performing_action = false;
        }
    }

    // Aplicar movimiento a la posición de la nave
    *translation += move_direction;

    // Actualizar la posición de la cámara para que siga a la nave
    camera.eye = *translation + orbit_radius * Vec3::new(rotation_y.sin(), 0.0, rotation_y.cos());
    camera.center = *translation;

    // Suavizar rotación y mantener orientación suave
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    // Actualizar posición en el minimapa
    minimap.update_ship_pos(translation.x, translation.z);
}

fn handle_autopilot(
    translation: &mut Vec3,
    rotation_y: &mut f32,
    rotation_x: &mut f32,
    camera: &mut Camera,
    minimap: &mut Minimap,
    simulated_keys: Vec<Key>,
) {
    let smooth_factor = 0.1;
    let movement_speed = 50.0;
    let rotation_speed = 0.05;
    let orbit_radius = 500.0;

    let mut move_direction = Vec3::new(0.0, 0.0, 0.0);

    // Orientación basada en la rotación actual
    let orientation = Vec3::new(rotation_y.sin(), 0.0, rotation_y.cos());

    let mut keys = simulated_keys;

    for key in keys {
        match key {
            Key::W => {
                move_direction += orientation * -movement_speed;
                *rotation_x = lerp(*rotation_x, -3.1, smooth_factor);
            },
            Key::A => {
                *rotation_y += rotation_speed;
            },
            Key::D => {
                *rotation_y -= rotation_speed;
            },
            _ => {}
        }
    }

    // Aplicar movimiento a la posición de la nave
    *translation += move_direction;

    // Actualizar la posición de la cámara para que siga a la nave
    camera.eye = *translation + orbit_radius * Vec3::new(rotation_y.sin(), 0.0, rotation_y.cos());
    camera.center = *translation;

    // Suavizar rotación y mantener orientación suave
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    // Actualizar posición en el minimapa
    minimap.update_ship_pos(translation.x, translation.z);
}

fn handle_camera(window: &Window, camera: &mut Camera) {
    if window.is_key_down(Key::Up) {
        camera.orbit(0.0,0.05);
    } 

    if window.is_key_down(Key::Down) {
        camera.orbit(0.0,-0.05);
    }

     if window.is_key_down(Key::Right) {
        camera.orbit(-0.05, 0.0);
    }
    if window.is_key_down(Key::Left) {
        camera.orbit(0.05, 0.0);
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

