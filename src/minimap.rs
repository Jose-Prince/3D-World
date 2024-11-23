//minimap.rs
use nalgebra_glm::Vec2;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::polygon::Polygon;

pub struct Minimap {
    height: isize,
    width: isize,
    position: Vec2,
}

impl Minimap {
    pub fn new(height: isize, width: isize, position: Vec2) -> Self {
        Minimap {
            height,
            width,
            position,
        }
    }

    pub fn render(&mut self, framebuffer: &mut Framebuffer) {
        let mut vertex = Vec::new();

        vertex.push([(self.position.x) as isize - self.width / 2, (self.position.y) as isize - self.height / 2]); //First vertex
        vertex.push([(self.position.x) as isize + (self.width / 2), (self.position.y) as isize - self.height / 2]); //Second vertex
        vertex.push([(self.position.x) as isize + (self.width / 2), (self.position.y) as isize + self.height / 2]); //Third vertex
        vertex.push([(self.position.x) as isize - (self.width / 2), (self.position.y) as isize + self.height / 2]); //Fourth vertex

        let border_color = Color::new(255,255,255);
        let inner_color = Color::new(255,255,255);

        framebuffer.polygon(&vertex, border_color, inner_color);

    }
}
