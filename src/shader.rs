//shader.rs
use nalgebra_glm::{Vec2, Vec3, Vec4, Mat3, mat4_to_mat3, dot};
use crate::vertex::Vertex;
use crate::render::Uniforms;
use crate::color::Color;
use crate::fragment::Fragment;
use std::f32::consts::PI;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    //Transform position
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );
    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    //Perform perspective division
    let w = transformed.w;
    let ndc_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    // apply viewport matrix
    let screen_position = uniforms.viewport_matrix * ndc_position;

    //Transform normal
    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    // Create a new Vertex with transformed attributes
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal,
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    spaceship_shader(fragment, uniforms)
    //planet_shader(fragment, uniforms)
    //earth_shader(fragment, uniforms)
    //magma_shader(fragment, uniforms)
    //combined_ice_cloud_shader(fragment, uniforms)
    //ice_shader(fragment, uniforms)
    //asteroid_shader(fragment, uniforms)
    //star_shader(fragment, uniforms)
    //mustafar_shader(fragment, uniforms)
    //fragment.color * fragment.intensity
    //stripe_shader(fragment, uniforms)
    //lava_shader(fragment, uniforms)
    //cloud_shader(fragment, uniforms)
    //transformed_shader(fragment, uniforms)
    //lerp_stripe_shader(fragment, uniforms)
    //wave_colot_shader(fragment, uniforms)
    //disco_ball_shader(fragment, uniforms)
    //moving_polka_dot_shader(fragment, uniforms)
    //moving_horizontal_stripes_shader(fragment, uniforms)
    //neon_light_shader(fragment)
    //core_shader(fragment)
    //glow_shader(fragment)
    //cellular_shader(fragment, uniforms)
    //purple_shader(fragment)
}

fn hull_layer(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Color base del casco metálico
    let base_color = Color::new(80, 80, 100); // Gris metálico
    let highlight_color = Color::new(200, 200, 220); // Resaltado

    // Simulación de reflejos metálicos con ruido
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x * 3.0, fragment.position.y * 3.0);
    let metallic_shine = base_color.lerp(&highlight_color, (noise_value * 0.5 + 0.5) as f32);

    // Intensidad de luz especular (simulada)
    let light_direction = Vec3::new(0.5, 0.5, 0.0).normalize();
    let fragment_normal = fragment.normal.normalize();
    let specular_intensity = light_direction.dot(&fragment_normal).clamp(0.0, 1.0);

    metallic_shine * (0.6 + 0.4 * specular_intensity) * fragment.intensity
}

fn engine_glow_layer(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Brillo de los motores (azul vibrante)
    let engine_glow_color = Color::new(50, 150, 255); // Azul eléctrico
    let noise_value = uniforms.noise.get_noise_3d(
        fragment.position.x * 10.0,
        fragment.position.y * 10.0,
        uniforms.time as f32 * 0.2,
    );

    let glow_intensity = (noise_value * 0.5 + 0.5).clamp(0.0, 1.0);
    engine_glow_color * glow_intensity * 0.8
}

fn scratches_layer(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Simulación de rayones en el casco
    let scratch_color = Color::new(30, 30, 40); // Oscuro, casi negro
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x * 20.0, fragment.position.y * 20.0);

    // Sólo aplicamos rayones en ciertas áreas
    let scratch_intensity = if noise_value > 0.6 { 1.0 } else { 0.0 };

    scratch_color * scratch_intensity
}

fn spaceship_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Capa del casco metálico
    let hull_color = hull_layer(fragment, uniforms);

    // Capa de brillo en los motores
    let engine_glow_color = engine_glow_layer(fragment, uniforms);

    // Capa de rayones
    let scratches_color = scratches_layer(fragment, uniforms);

    let light_direction = Vec3::new(0.5, 0.5, -0.5).normalize();
    let fragment_normal = fragment.normal.normalize();
    let diffuse_intensity = light_direction.dot(&fragment_normal).clamp(0.0, 1.0);

    let ambient_intensity = 0.5;
    let ambient_color = Color::new(80,80,100);

    let lighting = ambient_color * ambient_intensity + hull_color * diffuse_intensity;

    // Combinamos las capas
    let base_with_scratches = lighting.blend_with(&scratches_color);
    let blended_lighting = base_with_scratches.blend_with(&engine_glow_color);

    let min_intensity = 0.2;

    let final_color = blended_lighting * fragment.intensity + ambient_color * min_intensity;
    // Color final con intensidad de fragmento
    final_color
}

fn ocean_layer(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Color base para el océano (azul alienígena)
    let ocean_color = Color::new(0, 0, 150); 
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x * 5.0, fragment.position.y * 5.0);
    let intensity = (0.7 + 0.3 * noise_value) as f32; // Intensidad variada por el ruido
    ocean_color * intensity * fragment.intensity
}

fn continents_layer(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores para el terreno rocoso
    let base_color = Color::new(150, 75, 0);  // Color tierra marrón
    let rocky_color = Color::new(100, 100, 100); // Color gris para áreas rocosas

    // Ruido para simular montañas y valles
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x * 2.0, fragment.position.y * 2.0);
    let terrain_color = base_color.lerp(&rocky_color, (noise_value * 0.5 + 0.5) as f32);

    terrain_color * fragment.intensity
}

fn atmosphere_layer(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores de la atmósfera (suave brillo)
    let atmosphere_color = Color::new(50, 100, 200); // Un azul brillante
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x * 0.5, fragment.position.y * 0.5);
    let intensity = (0.5 + 0.5 * noise_value) as f32; // Atmósfera con variaciones sutiles
    atmosphere_color * intensity * fragment.intensity
}

fn clouds_layer(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Colores de las nubes (blanco y suave)
    let cloud_color = Color::new(255, 255, 255);
    let cloud_noise = uniforms.noise.get_noise_3d(
        fragment.position.x * 2.0 + uniforms.time as f32 * 0.1,
        fragment.position.y * 2.0 + uniforms.time as f32 * 0.1,
        fragment.depth
    );

    let cloud_intensity = ((cloud_noise * 0.5 + 0.5) * 0.5).clamp(0.0, 0.3) as f32; // Nubes sutiles
    cloud_color * cloud_intensity * 0.7
}

fn planet_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Capa del océano
    let ocean_color = ocean_layer(fragment, uniforms);
    
    // Capa de los continentes (terreno rocoso)
    let continents_color = continents_layer(fragment, uniforms);

    // Capa de la atmósfera
    let atmosphere_color = atmosphere_layer(fragment, uniforms);

    // Capa de nubes
    let clouds_color = clouds_layer(fragment, uniforms);

    // Combinamos las capas en un orden adecuado
    let base_color = if continents_color.is_equal(&ocean_color) {
        continents_color
    } else {
        ocean_color
    };

    // Primero combinamos las nubes con el planeta
    let planet_with_clouds = clouds_color.blend_with(&base_color);

    // Luego combinamos la atmósfera
    let final_color = atmosphere_color.blend_with(&planet_with_clouds);

    // Devuelve el color final con la intensidad de fragmento
    final_color 
}

fn ocean_layer2(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ocean_color = Color::new(0, 105, 148);
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x * 10.0, fragment.position.y * 10.0);
    let intensity = (0.8 + 0.2 * noise_value) as f32;
    ocean_color * intensity * fragment.intensity
}

fn continents_layer2(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let land_color = Color::new(34,139,34);
    let desert_color = Color::new(194, 178, 128);
    let noise_value = uniforms.noise.get_noise_2d(fragment.position.x * 5.0, fragment.position.y * 5.0);
    land_color.lerp(&desert_color, (noise_value * 0.5 + 0.5) as f32) * fragment.intensity
}

fn clouds_layer2(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let cloud_color = Color::new(255,255,255);
    let cloud_noise = uniforms.noise.get_noise_3d(
        fragment.position.x * 2.0 + uniforms.time as f32 * 0.1,
        fragment.position.y * 2.0 + uniforms.time as f32 * 0.1,
        fragment.depth
    );

    let cloud_intensity = ((cloud_noise * 0.5 +0.5) * 0.8).clamp(0.0, 0.1) as f32;
    cloud_color * cloud_intensity * 0.7
}

fn earth_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ocean_color = ocean_layer2(fragment, uniforms);
    let continents_color = continents_layer2(fragment, uniforms);
    let clouds_color = clouds_layer2(fragment, uniforms);

    if !continents_color.is_equal(&ocean_color) {
        let land_or_ocean = continents_color;

        if clouds_color.is_black() {
            land_or_ocean
        } else {
            clouds_color.blend_with(&land_or_ocean)
        }
    } else {
        if clouds_color.is_black() {
            ocean_color
        } else {
            clouds_color.blend_with(&ocean_color)
        }
    }
}

pub fn magma_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let bright_color = Color::new(255, 100, 0);
    let dark_color = Color::new(50,10,0);

    let position = Vec3::new(
        fragment.vertex_position.x,
        fragment.vertex_position.y,
        fragment.depth,
    );

    let base_frequency = 0.3;
    let pulsate_amplitude = 0.7;
    let t = uniforms.time as f32 * 0.02;

    let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

    let zoom = 800.0;
    let noise_value1 = uniforms.noise.get_noise_3d(
        position.x * zoom,
        position.y * zoom,
        (position.z + pulsate) * zoom,
    );

    let noise_value2 = uniforms.noise.get_noise_3d(
        (position.x +1000.0) * zoom,
        (position.y +1000.0) * zoom,
        (position.z + 2000.0 + pulsate) * zoom,
    );

    let noise_value = (noise_value1 + noise_value2) * 0.5;

    let color = dark_color.lerp(&bright_color, noise_value);

    color  
}

pub fn combined_ice_cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let ice_color = ice_shader(fragment, uniforms);
    let cloud_color = cloud_ice_shader(fragment, uniforms);

    if !cloud_color.is_black() {
        cloud_color * fragment.intensity + ice_color * 0.7
    } else {
        ice_color * fragment.intensity
    }
}

pub fn cloud_ice_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let uv = fragment.position;

    let speed = 0.01;
    let scale = 3.0;

    let moving_x = uv.x * scale + uniforms.time as f32 * speed;
    let noise_value = uniforms.noise.get_noise_2d(moving_x, uv.y * scale);

    let cloud_intensity = (200.0 * (noise_value * 0.5 +0.5)).clamp(0.0, 255.0);

    Color::new(
        cloud_intensity as i32,
        cloud_intensity as i32,
        cloud_intensity as i32,
    )
}

pub fn ice_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let uv = fragment.position;

    let noise_value = uniforms.noise.get_noise_2d(uv.x * 4.0, uv.y * 4.0);

    let blue_intensity = (180.0 + 60.0 * (noise_value * 0.5 + 0.5)).clamp(0.0, 255.0);
    let white_intensity = (200.0 + 30.0 * (noise_value * 0.5 + 0.5)).clamp(180.0, 255.0);

    Color::new(
        white_intensity as i32,
        (white_intensity * 0.95) as i32,
        blue_intensity as i32,
    )
}

pub fn asteroid_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let uv = fragment.position;

    let noise_value = uniforms.noise.get_noise_2d(uv.x * 10.0, uv.y * 10.0);

    let base_color_intensity = (80.0 + 40.0 * fragment.intensity * (noise_value * 0.5 + 0.5)).clamp(0.0, 120.0);

    Color::new(
        base_color_intensity as i32,
        (base_color_intensity * 0.9) as i32,
        (base_color_intensity * 0.7) as i32,
    )
}

pub fn star_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let uv = fragment.position;
    let noise_value = uniforms.noise.get_noise_2d(uv.x * 5.0, uv.y * 5.0);

    let red_intensity = 255 - (50.0 * (noise_value * 0.5 + 0.5)).clamp(0.0, 50.0) as i32;
    let green_intensity = 180 - (50.0 * (noise_value * 0.5 + 0.5)).clamp(0.0, 50.0) as i32;
   Color::new(
        (red_intensity ) as i32,
        (green_intensity ) as i32,
        (40.0 ) as i32,
    ) 
}

pub fn mustafar_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    //UV coordinates
    let uv = fragment.position;

    //noise animation
    let time_factor = uniforms.time as f32 * 0.01;

    //cellular noise for the cracks
    let crack_noise = uniforms.noise.get_noise_2d(uv.x * 0.3, uv.y * 0.3 + time_factor);

    //magma noise
    let magma_noise = uniforms.noise.get_noise_2d(uv.x * 1.1 + time_factor, uv.y * 1.1 - time_factor);

    //Mapping the noise of magma color
    let lava_intensity = ((magma_noise * 0.5 + 0.5) * fragment.intensity * 255.0) as u8;
    let crack_intensity = ((crack_noise * 0.5 +0.5) * 255.0) as u8;

    let shadow_factor = (fragment.depth * 0.8 + fragment.normal.z * 0.2) as f32;
    let shadow_color = (30.0 * shadow_factor) as u8;

    if crack_intensity < 100 {
        Color::new(shadow_color as i32, (crack_intensity / 2) as i32, shadow_color as i32)
    } else if crack_intensity < 150 {
        Color::new(120, (lava_intensity / 2) as i32, 40)
    } else {
        Color::new(255, lava_intensity as i32, (lava_intensity / 3) as i32)
    }
}

pub fn stripe_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let y = fragment.position.y as usize;

    //Define stripe colors
    let colors = [
        Color::new(255,0,0),
        Color::new(0,255,0),
        Color::new(0,0,255),
        Color::new(255,255,0),
    ];

    //Define stripe width
    let stripe_width = 20;

    //Calculate which stripe this fragment belongs to
    let stripe_index = (y / stripe_width) % colors.len();

    //Return the color for this stripe
    colors[stripe_index]
}

pub fn transformed_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let stripe_coord = fragment.vertex_position.y;

    //Define stripe colors
    let colors = [
        Color::new(255,0,0),
        Color::new(0,255,0),
        Color::new(0,0,255),
        Color::new(255,255,0),

    ];

    //Define stripe width
    let stripe_width = 0.1;

    //Calculate which stripe this fragment belongs to
    let stripe_index = ((stripe_coord / stripe_width).abs() as usize) % colors.len();

    //Return the color for this stripe
    colors[stripe_index] * fragment.intensity
}

pub fn lerp_stripe_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    //Define stripe colors
    let colors = [
        Color::new(255,0,0),
        Color::new(0,255,0),
        Color::new(0,0,255),
        Color::new(255,255,0),

    ];

    //Define stripe width
    let stripe_width = 0.1;

    //Use the y-coordinate of the transformed position for stripe calculation
    let stripe_coord = fragment.vertex_position.y;

    //Calculate which stripe this fragment belongs to
    let stripe_float = (stripe_coord / stripe_width).abs();
    let stripe_index = (stripe_float as usize) & colors.len();
    let next_index = (stripe_index + 1) % colors.len();

    //Calculate interpolation factor
    let t = stripe_float.fract();

    //Interpolation between current and next color
    colors[stripe_index].lerp(&colors[next_index], t) * fragment.intensity
}

pub fn wave_colot_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let color1 = Color::new(255,0,0);
    let color2 = Color::new(0,255,0);
    let color3 = Color::new(0,0,255);

    //Use both x and y coordinates for more interesting patterns
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let frequency = 10.0;

    //Create three overlaping sine wave
    let wave1 = (x *7.0 * frequency + y * 5.0 * frequency).sin() * 0.5 + 0.5;
    let wave2 = (x * 5.0 * frequency - y * 8.0 * frequency + PI / 3.0).sin() * 0.5 + 0.5;
    let wave3 = (y * 6.0 * frequency + x * 4.0 * frequency + 2.0 * PI / 3.0).sin() *0.5 -0.5;

    //Interpolate between colors based on the wave values 
    let mut final_color = color1.lerp(&color2, wave1);
    final_color = final_color.lerp(&color3, wave2);
    final_color = final_color.lerp(&color1, wave3);

    final_color * fragment.intensity
}

pub fn time_based_color_cycling_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Define a list of colors to cycle through
  let colors = [
    Color::new(255, 0, 0),    // Red
    Color::new(0, 255, 0),    // Green
    Color::new(0, 0, 255),    // Blue
    Color::new(255, 255, 0),  // Yellow
    Color::new(255, 0, 255),  // Magenta
    Color::new(0, 255, 255),  // Cyan
  ];

  // Define how many frames to show each color
  let frames_per_color = 100;

  // Calculate which color we should be showing
  let color_index = (uniforms.time / frames_per_color) as usize % colors.len();

  // Calculate how far we are into the transition to the next color
  let transition_progress = (uniforms.time % frames_per_color) as f32 / frames_per_color as f32;

  // Get the current color and the next color
  let current_color = colors[color_index];
  let next_color = colors[(color_index + 1) % colors.len()];

  // Interpolate between the current color and the next color
  current_color.lerp(&next_color, transition_progress) * fragment.intensity
}

pub fn moving_horizontal_stripes_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Define stripe colors
  let color1 = Color::new(255, 0, 0);   // Red
  let color2 = Color::new(0, 0, 255);   // Blue

  // Define stripe properties
  let stripe_width = 0.2;  // Width of each stripe
  let speed = 0.002;        // Speed of stripe movement

  // Calculate the y-coordinate for the moving effect
  let moving_y = fragment.vertex_position.y + uniforms.time as f32 * speed;

  // Create the stripe pattern
  let stripe_factor = ((moving_y / stripe_width) * PI).sin() * 0.5 + 0.5;

  // Interpolate between the two colors based on the stripe factor
  color1.lerp(&color2, stripe_factor) * fragment.intensity
}

pub fn moving_polka_dot_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Define colors
  let background_color = Color::new(250, 250, 250);  // Light gray
  let dot_color = Color::new(255, 0, 0);        // Red

  // Define dot properties
  let dot_size = 0.1;
  let dot_spacing = 0.3;
  let speed = 0.001;

  // Calculate moving coordinates
  let moving_x = fragment.vertex_position.x + uniforms.time as f32 * speed;
  let moving_y = fragment.vertex_position.y - uniforms.time as f32 * speed * 0.5;

  // Create dot pattern using sine and cosine
  let pattern_x = ((moving_x / dot_spacing) * 2.0 * PI).cos();
  let pattern_y = ((moving_y / dot_spacing) * 2.0 * PI).cos();

  // Combine patterns to create dots
  let dot_pattern = (pattern_x * pattern_y).max(0.0);

  // Apply dot size
  let dot_factor = (dot_pattern - (1.0 - dot_size)).max(0.0) / dot_size;

  // Interpolate between background color and dot color
  background_color.lerp(&dot_color, dot_factor) * fragment.intensity
}


pub fn disco_ball_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
      // Base color for the disco ball (silver-like)
    let base_color = Color::new(100, 100, 210);
    
    // Light color (bright white)
    let light_color = Color::new(255, 255, 255);

    // Parameters for the tile pattern
    let tile_freq_x = 20.0;
    let tile_freq_y = 40.0;  // Increased frequency for vertical lines
    let tile_freq_z = 20.0;
    let tile_scale = 0.05;

    // Parameters for the moving lights
    let light_speed = 0.05;
    let num_lights = 5;
    let light_size = 0.15;  // Increased for visibility

    // Create tile pattern
    let x = fragment.vertex_position.x;
    let y = fragment.vertex_position.y;
    let z = fragment.vertex_position.z;

    // Modified tile pattern calculation for more vertical lines
    let tile_pattern = (
        ((x * tile_freq_x).sin().abs() * 0.5 + 0.5) * 
        ((y * tile_freq_y).sin().abs() * 0.8 + 0.2) * 
        ((z * tile_freq_z).sin().abs() * 0.5 + 0.5) * 
        tile_scale
    ).min(1.0);

    // Calculate normal for simple lighting
    let normal = fragment.normal.normalize();
    let light_dir = Vec3::new(0.0, 0.0, -1.0); // Light coming from the camera
    let light_intensity = dot(&normal, &light_dir).max(0.0);

    // Create moving light spots
    let mut light_factor = 0.0;
    for i in 0..num_lights {
        let angle = 2.0 * PI * (i as f32 / num_lights as f32) + uniforms.time as f32 * light_speed;
        let light_x = (angle.cos() * 0.5 + 0.5) * 0.8 + 0.1;  // Scale and offset to fit in 0-1 range
        let light_y = (angle.sin() * 0.5 + 0.5) * 0.8 + 0.1;  // Scale and offset to fit in 0-1 range
        
        let dist = ((x - light_x).powi(2) + (y - light_y).powi(2)).sqrt();
        light_factor += (1.0 - (dist / light_size).min(1.0)).max(0.0);
    }
    light_factor = light_factor.min(1.0);

    // Combine base color, tile pattern, lighting, and moving lights
    let tile_color = base_color.lerp(&light_color, tile_pattern * light_intensity);
    tile_color.lerp(&light_color, light_factor * 0.7) * fragment.intensity
}

fn static_pattern_shader(fragment: &Fragment) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let pattern = ((x * 10.0).sin() * (y * 10.0).sin()).abs();

  let r = (pattern * 255.0) as i32;
  let g = ((1.0 - pattern) * 255.0) as i32;
  let b = 128;

  Color::new(r, g, b)
}

fn moving_circles_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let time = uniforms.time as f32 * 0.05;
  let circle1_x = (time.sin() * 0.4 + 0.5) % 1.0;
  let circle2_x = (time.cos() * 0.4 + 0.5) % 1.0;

  let dist1 = ((x - circle1_x).powi(2) + (y - 0.3).powi(2)).sqrt();
  let dist2 = ((x - circle2_x).powi(2) + (y - 0.7).powi(2)).sqrt();

  let circle_size = 0.1;
  let circle1 = if dist1 < circle_size { 1.0f32 } else { 0.0f32 };
  let circle2 = if dist2 < circle_size { 1.0f32 } else { 0.0f32 };

  let circle_intensity = (circle1 + circle2).min(1.0f32);

  Color::new(
    (circle_intensity * 255.0) as i32,
    (circle_intensity * 255.0) as i32,
    (circle_intensity * 255.0) as i32
  )
}

// Combined shader
pub fn combined_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let base_color = static_pattern_shader(fragment);
  let circle_color = moving_circles_shader(fragment, uniforms);

  // Combine shaders: use circle color if it's not black, otherwise use base color
  if !circle_color.is_black() {
    circle_color * fragment.intensity
  } else {
    base_color * fragment.intensity
  }
}

// Simple purple shader
fn purple_shader(_fragment: &Fragment) -> Color {
  Color::new(128, 0, 128) // Purple color
}

// Circle shader
fn circle_shader(fragment: &Fragment) -> Color {
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let distance = (x * x + y * y).sqrt();

  if distance < 0.25 { // Circle radius
    Color::new(255, 255, 0) // Yellow circle
  } else {
    Color::new(0, 0, 0) // Black (transparent) background
  }
}

// Combined shader with blend mode parameter
pub fn combined_blend_shader(fragment: &Fragment, blend_mode: &str) -> Color {
  let base_color = purple_shader(fragment);
  let circle_color = circle_shader(fragment);

  let combined_color = match blend_mode {
    "normal" => base_color.blend_normal(&circle_color),
    "multiply" => base_color.blend_multiply(&circle_color),
    "add" => base_color.blend_add(&circle_color),
    "subtract" => base_color.blend_subtract(&circle_color),
    _ => base_color // Default to base color if unknown blend mode
  };

  combined_color * fragment.intensity
}

fn glow_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let glow_size = 0.05; 
    
    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let glow_intensity = ((1.0 - (distance_to_center / glow_size).min(1.0)) * PI / 2.0).sin();
    
    // Neon blue color for the glow
    Color::new(
        (0.0 * glow_intensity * 255.0) as i32,
        (0.5 * glow_intensity * 255.0) as i32,
        (glow_intensity * 255.0) as i32
    )
}

fn core_shader(fragment: &Fragment) -> Color {
    let y = fragment.vertex_position.y;
    let stripe_width = 0.2;
    let core_size = 0.02;
    
    let distance_to_center = (y % stripe_width - stripe_width / 2.0).abs();
    let core_intensity = if distance_to_center < core_size { 1.0 } else { 0.0 };
    
    Color::new(
        (0.8 * core_intensity * 255.0) as i32,
        (0.9 * core_intensity * 255.0) as i32,
        (core_intensity * 255.0) as i32
    )
}

fn background_shader(_fragment: &Fragment) -> Color {
    Color::new(10, 10, 20) // Dark blue background
}

// Combined neon light shader
pub fn neon_light_shader(fragment: &Fragment) -> Color {
    let background = background_shader(fragment);
    let glow = glow_shader(fragment);
    let core = core_shader(fragment);
    
    // Blend the glow with the background using "screen" blend mode
    let blended_glow = background.blend_screen(&glow);
    
    // Add the core on top using "add" blend mode
    blended_glow.blend_add(&core) * fragment.intensity
}


fn random_color_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let seed = uniforms.time as u64;

  let mut rng = StdRng::seed_from_u64(seed);

  let r = rng.gen_range(0..=255);
  let g = rng.gen_range(0..=255);
  let b = rng.gen_range(0..=255);

  let random_color = Color::new(r, g, b);

  random_color * fragment.intensity
}

fn black_and_white(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let seed = uniforms.time as f32 * fragment.vertex_position.y * fragment.vertex_position.x;

  let mut rng = StdRng::seed_from_u64(seed.abs() as u64);

  let random_number = rng.gen_range(0..=100);

  let black_or_white = if random_number < 50 {
    Color::new(0, 0, 0)
  } else {
    Color::new(255, 255, 255)
  };

  black_or_white * fragment.intensity
}

fn dalmata_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;
  let ox = 0.0;
  let oy = 0.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  let noise_value = uniforms.noise.get_noise_2d(
    (x + ox) * zoom,
    (y + oy) * zoom,
  );

  let spot_threshold = 0.5;
  let spot_color = Color::new(255, 255, 255); // White
  let base_color = Color::new(0, 0, 0); // Black

  let noise_color = if noise_value < spot_threshold {
    spot_color
  } else {
    base_color
  };

  noise_color * fragment.intensity
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;  // to move our values 
  let ox = 100.0; // offset x in the noise map
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5;

  let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

  // Define cloud threshold and colors
  let cloud_threshold = 0.5; // Adjust this value to change cloud density
  let cloud_color = Color::new(255, 255, 255); // White for clouds
  let sky_color = Color::new(30, 97, 145); // Sky blue

  // Determine if the pixel is part of a cloud or sky
  let noise_color = if noise_value > cloud_threshold {
    cloud_color
  } else {
    sky_color
  };

  noise_color * fragment.intensity
}

fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
  let ox = 50.0;    // Offset x in the noise map
  let oy = 50.0;    // Offset y in the noise map
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Use a cellular noise function to create the plant cell pattern
  let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();

  // Define different shades of green for the plant cells
  let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
  let cell_color_2 = Color::new(124, 252, 0);   // Light green
  let cell_color_3 = Color::new(34, 139, 34);   // Forest green
  let cell_color_4 = Color::new(173, 255, 47);  // Yellow green

  // Use the noise value to assign a different color to each cell
  let final_color = if cell_noise_value < 0.15 {
    cell_color_1
  } else if cell_noise_value < 0.7 {
    cell_color_2
  } else if cell_noise_value < 0.75 {
    cell_color_3
  } else {
    cell_color_4
  };

  // Adjust intensity to simulate lighting effects (optional)
  final_color * fragment.intensity
}

fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // Base colors for the lava effect
  let bright_color = Color::new(255, 240, 0); // Bright orange (lava-like)
  let dark_color = Color::new(130, 20, 0);   // Darker red-orange

  // Get fragment position
  let position = Vec3::new(
    fragment.vertex_position.x,
    fragment.vertex_position.y,
    fragment.depth
  );

  // Base frequency and amplitude for the pulsating effect
  let base_frequency = 0.2;
  let pulsate_amplitude = 0.5;
  let t = uniforms.time as f32 * 0.01;

  // Pulsate on the z-axis to change spot size
  let pulsate = (t * base_frequency).sin() * pulsate_amplitude;

  // Apply noise to coordinates with subtle pulsating on z-axis
  let zoom = 1000.0; // Constant zoom factor
  let noise_value1 = uniforms.noise.get_noise_3d(
    position.x * zoom,
    position.y * zoom,
    (position.z + pulsate) * zoom
  );
  let noise_value2 = uniforms.noise.get_noise_3d(
    (position.x + 1000.0) * zoom,
    (position.y + 1000.0) * zoom,
    (position.z + 1000.0 + pulsate) * zoom
  );
  let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions

  // Use lerp for color blending based on noise value
  let color = dark_color.lerp(&bright_color, noise_value);

  color * fragment.intensity
}
