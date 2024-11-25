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

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, number: u8) -> Color {
    if number == 0 {
        spaceship_shader(fragment, uniforms)
    } else if number == 1 {
        earth_shader(fragment, uniforms)
    } else if number == 2 {
        magma_shader(fragment, uniforms)
    } else if number == 3 {
        combined_ice_cloud_shader(fragment, uniforms)
    } else if number == 4 {
        star_shader(fragment, uniforms)
    } else if number == 5 {
        lava_shader(fragment, uniforms)
    } else {
        planet_shader(fragment, uniforms)
    }
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
