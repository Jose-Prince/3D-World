//render.rs
use nalgebra_glm::{Mat4, Vec2, Vec3, Vec4};
use crate::vertex::Vertex;

pub struct Uniforms {
    pub model_matrix: Mat4,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );

    transform_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    //Vertex Shader Stage
    let mut transformed_vertices = Vec::new();
    for vertex in vertex_array {
        let transformed_vertex = vertex_shader(&vertex, &uniforms);
        transformed_vertices.push(transformed_vertex);
    }

    //Primitive Assembly Stage
    for chunk in transformed_vertices.chunks(3) {
        if chunk.len() == 3 {
            let v0 = &chunk[0];
            let v1 = &chunk[1];
            let v2 = &chunk[2];
        }
    }

    //Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangules {
        fragments.extend(triangule(&tri[0], &tri[1], &tri[2]));
    }


    //Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        let color = fragment.color.to_hex();
        framebuffer.set_current_color(color);
        framebuffer.point(x, y);
    } 
}
