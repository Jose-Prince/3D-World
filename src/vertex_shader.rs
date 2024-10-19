// vertex_shader.rs
use nalgebra_glm::{Vec3, Vec4, Mat3};
use crate::vertex::Vertex;
use crate::render::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Step 1: Transform the vertex position from model space to world space
    let model_pos = uniforms.model_matrix * Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    // Step 2: Transform from world space to view space
    let view_pos = uniforms.view_matrix * model_pos;

    // Step 3: Transform from view space to clip space
    let clip_pos = uniforms.projection_matrix * view_pos;

    // Step 4: Perform perspective division to get normalized device coordinates (NDC)
    let ndc_pos = if clip_pos.w.abs() > 1e-5 {
        Vec3::new(
            clip_pos.x / clip_pos.w,
            clip_pos.y / clip_pos.w,
            clip_pos.z / clip_pos.w
        )
    } else {
        Vec3::new(clip_pos.x, clip_pos.y, clip_pos.z)
    };

    // Step 5: Apply viewport transformation to get screen coordinates
    let screen_pos = uniforms.viewport_matrix * Vec4::new(ndc_pos.x, ndc_pos.y, ndc_pos.z, 1.0);

    // Step 6: Transform the normal
    let model_matrix_3x3 = Mat3::new(
        uniforms.model_matrix[0], uniforms.model_matrix[1], uniforms.model_matrix[2],
        uniforms.model_matrix[4], uniforms.model_matrix[5], uniforms.model_matrix[6],
        uniforms.model_matrix[8], uniforms.model_matrix[9], uniforms.model_matrix[10]
    );
    let normal_matrix = model_matrix_3x3.transpose().try_inverse().unwrap_or(Mat3::identity());
    let transformed_normal = normal_matrix * vertex.normal;

    // Create a new Vertex with transformed attributes
    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_pos.x, screen_pos.y, screen_pos.z),
        transformed_normal,
    }
}
