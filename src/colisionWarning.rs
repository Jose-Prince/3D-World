use crate::Color;
use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use nalgebra_glm::Vec2;

pub struct ColisionWarning {
    big_msg: String,
    small_msg: String,
    color_back: Color,
    color_warning: Color,
}

impl ColisionWarning {
    pub fn new(
        big_msg: String,
        small_msg: String,
        color_back: Color,
        color_warning: Color,
    ) -> Self {
        ColisionWarning {
            big_msg,
            small_msg,
            color_back,
            color_warning
        }
    }

    pub fn render(&mut self, framebuffer: &mut Framebuffer) {
        let mut vertex = Vec::new();
        let mut vertex_back = Vec::new();

        vertex.push([(framebuffer.width / 2) as isize - (framebuffer.width / 8) as isize, (framebuffer.height / 2)  as isize - (framebuffer.height / 16) as isize]); //First vertex
        vertex.push([(framebuffer.width / 2) as isize + (framebuffer.width / 8) as isize, (framebuffer.height / 2)  as isize - (framebuffer.height / 16) as isize]); //Second vertex
        vertex.push([(framebuffer.width / 2) as isize + (framebuffer.width / 8) as isize, (framebuffer.height / 2)  as isize + (framebuffer.height / 16) as isize]); //Third vertex
        vertex.push([(framebuffer.width / 2) as isize - (framebuffer.width / 8) as isize, (framebuffer.height / 2)  as isize + (framebuffer.height / 16) as isize]); //Fourth vertex
        
        let margin = 2; // Desplazamiento uniforme para agrandar
vertex_back.push([
    (framebuffer.width / 2) as isize - (framebuffer.width / 8 + margin) as isize, 
    (framebuffer.height / 2) as isize - (framebuffer.height / 16 + margin) as isize
]);
vertex_back.push([
    (framebuffer.width / 2) as isize + (framebuffer.width / 8 + margin) as isize, 
    (framebuffer.height / 2) as isize - (framebuffer.height / 16 + margin) as isize
]);
vertex_back.push([
    (framebuffer.width / 2) as isize + (framebuffer.width / 8 + margin) as isize, 
    (framebuffer.height / 2) as isize + (framebuffer.height / 16 + margin) as isize
]);
vertex_back.push([
    (framebuffer.width / 2) as isize - (framebuffer.width / 8 + margin) as isize, 
    (framebuffer.height / 2) as isize + (framebuffer.height / 16 + margin) as isize
]);


        framebuffer.polygon(&vertex, self.color_warning, self.color_warning);
        framebuffer.polygon(&vertex_back, self.color_back, self.color_back); 
    }
}
