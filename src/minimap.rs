//minimap.rs
use nalgebra_glm::Vec2;
use crate::framebuffer::Framebuffer;
use crate::color::Color;
use crate::polygon::Polygon;

pub struct Minimap {
    height: isize,
    width: isize,
    position: Vec2,
    ship_pos: Vec2,
}

impl Minimap {
    pub fn new(
        height: isize, 
        width: isize, 
        position: Vec2, 
        ship_pos: Vec2, 
    ) -> Self {
        Minimap {
            height,
            width,
            position,
            ship_pos,
        }
    }

    pub fn render(&mut self, framebuffer: &mut Framebuffer) {
        let mut vertex = Vec::new();
        let mut vertex_back = Vec::new();

        vertex.push([(self.position.x) as isize - self.width / 2, (self.position.y) as isize - self.height / 2]); //First vertex
        vertex.push([(self.position.x) as isize + (self.width / 2), (self.position.y) as isize - self.height / 2]); //Second vertex
        vertex.push([(self.position.x) as isize + (self.width / 2), (self.position.y) as isize + self.height / 2]); //Third vertex
        vertex.push([(self.position.x) as isize - (self.width / 2), (self.position.y) as isize + self.height / 2]); //Fourth vertex
        
        vertex_back.push([(self.position.x) as isize - self.width / 2 - 2, (self.position.y) as isize - self.height / 2 - 2]); //First vertex
        vertex_back.push([(self.position.x) as isize + (self.width / 2) + 2, (self.position.y) as isize - self.height / 2 - 2]); //Second vertex
        vertex_back.push([(self.position.x) as isize + (self.width / 2) + 2, (self.position.y) as isize + self.height / 2 + 2]); //Third vertex
        vertex_back.push([(self.position.x) as isize - (self.width / 2) - 2, (self.position.y) as isize + self.height / 2 + 2]); //Fourth vertex

        let border_color = Color::new(3,252,53);
        let inner_color = Color::new(0,0,0);

        framebuffer.set_current_color(Color::new(255,0,0));

        //Transform ship translation to minimap coords
        let coord_x = self.position.x as isize + (self.ship_pos.x / 100.0) as isize;
        let coord_y = self.position.y as isize + (self.ship_pos.y / 100.0) as isize;

        framebuffer.point(coord_x, coord_y, 0.0);

        framebuffer.polygon(&vertex, border_color, inner_color);
        framebuffer.polygon(&vertex_back, border_color, border_color);
    }

    pub fn update_ship_pos(&mut self, pos_x : f32, pos_y : f32) {
        self.ship_pos = Vec2::new(pos_x, pos_y);
    }
}
