pub struct Uniforms {

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
    for vertex in vertex_array {
        vertex_shader(&vertex, &uniforms);
    }

    //Primitive Assembly Stage

    //Rasterization Stage

    //Fragment Processing Stage
    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;

        let color = fragment.color.to_hex();
        framebuffer.set_current_color(color);
        framebuffer.point(x, y);
    } 
}

pub fn 