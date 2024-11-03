// vertex_shader.rs
use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3, dot};
use crate::vertex::Vertex;
use crate::render::Uniforms;
use crate::color::Color;
use crate::fragment::Fragment;
use std::f32::consts::PI;

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
    
    //fragment.color * fragment.intensity
    //stripe_shader(fragment, uniforms)
    //transformed_shader(fragment, uniforms)
    //lerp_stripe_shader(fragment, uniforms)
    //wave_colot_shader(fragment, uniforms)
    //disco_ball_shader(fragment, uniforms)
    //moving_polka_dot_shader(fragment, uniforms)
    moving_horizontal_stripes_shader(fragment, uniforms)
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
