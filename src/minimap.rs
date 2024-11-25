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
    planet1_pos: Vec2,
    planet2_pos: Vec2,
    planet3_pos: Vec2,
    planet4_pos: Vec2,
    planet5_pos: Vec2,
}

impl Minimap {
    pub fn new(
        height: isize, 
        width: isize, 
        position: Vec2, 
        ship_pos: Vec2, 
        planet1_pos: Vec2,
        planet2_pos: Vec2,
        planet3_pos: Vec2,
        planet4_pos: Vec2,
        planet5_pos: Vec2,
    ) -> Self {
        Minimap {
            height,
            width,
            position,
            ship_pos,
            planet1_pos,
            planet2_pos,
            planet3_pos,
            planet4_pos,
            planet5_pos
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
        let mut coord_x = self.position.x as isize + (self.ship_pos.x / 100.0) as isize;
        let mut coord_y = self.position.y as isize + (self.ship_pos.y / 100.0) as isize;

        framebuffer.point(coord_x, coord_y, 0.0);

        //Star coordinates
        framebuffer.set_current_color(Color::new(255,255,255));

        framebuffer.point((self.position.x - 1.0) as isize, (self.position.y - 1.0) as isize, 0.0);
        framebuffer.point((self.position.x - 1.0) as isize, self.position.y as isize, 0.0);
        framebuffer.point((self.position.x - 1.0) as isize, (self.position.y + 1.0) as isize, 0.0);
        framebuffer.point(self.position.x as isize, (self.position.y - 1.0) as isize, 0.0);
        framebuffer.point(self.position.x as isize, self.position.y as isize, 0.0);
        framebuffer.point(self.position.x as isize, (self.position.y + 1.0) as isize, 0.0);
        framebuffer.point((self.position.x + 1.0) as isize, (self.position.y - 1.0) as isize, 0.0);
        framebuffer.point((self.position.x + 1.0) as isize, self.position.y as isize, 0.0);
        framebuffer.point((self.position.x + 1.0) as isize, (self.position.y + 1.0) as isize, 0.0);

        //Planet 1 coords
        framebuffer.set_current_color(Color::new(79,22,19));
        coord_x = self.position.x as isize + (self.planet1_pos.x / 100.0) as isize;
        coord_y = self.position.y as isize + (self.planet1_pos.y / 100.0) as isize;

        framebuffer.point((coord_x - 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, coord_y as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y + 1) as isize, 0.0);
        
        //Planet 2 coords
        framebuffer.set_current_color(Color::new(109,218,222));
        coord_x = self.position.x as isize + (self.planet2_pos.x / 100.0) as isize;
        coord_y = self.position.y as isize + (self.planet2_pos.y / 100.0) as isize;

        framebuffer.point((coord_x - 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, coord_y as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y + 1) as isize, 0.0);
        
        
        //Planet 3 coords
        framebuffer.set_current_color(Color::new(87,179,82));
        coord_x = self.position.x as isize + (self.planet3_pos.x / 100.0) as isize;
        coord_y = self.position.y as isize + (self.planet3_pos.y / 100.0) as isize;

        framebuffer.point((coord_x - 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, coord_y as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y + 1) as isize, 0.0);
        
        //Planet 4 coords
        framebuffer.set_current_color(Color::new(125,19,14));
        coord_x = self.position.x as isize + (self.planet4_pos.x / 100.0) as isize;
        coord_y = self.position.y as isize + (self.planet4_pos.y / 100.0) as isize;

        framebuffer.point((coord_x - 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, coord_y as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y + 1) as isize, 0.0);
        
        //Planet 5 coords
        framebuffer.set_current_color(Color::new(2,5,64));
        coord_x = self.position.x as isize + (self.planet5_pos.x / 100.0) as isize;
        coord_y = self.position.y as isize + (self.planet5_pos.y / 100.0) as isize;

        framebuffer.point((coord_x - 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x - 1) as isize as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point(coord_x as isize, coord_y as isize, 0.0);
        framebuffer.point(coord_x as isize, (coord_y + 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y - 1) as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, coord_y as isize, 0.0);
        framebuffer.point((coord_x + 1) as isize, (coord_y + 1) as isize, 0.0);

        framebuffer.polygon(&vertex, border_color, inner_color);
        framebuffer.polygon(&vertex_back, border_color, border_color);
    }

    pub fn update_ship_pos(&mut self, pos_x : f32, pos_y : f32) {
        self.ship_pos = Vec2::new(pos_x, pos_y);
    }

    pub fn update_p1_pos(&mut self, pos_x : f32, pos_y : f32) {
        self.planet1_pos = Vec2::new(pos_x, pos_y);
    }

    pub fn update_p2_pos(&mut self, pos_x : f32, pos_y : f32) {
        self.planet2_pos = Vec2::new(pos_x, pos_y);
    }

    pub fn update_p3_pos(&mut self, pos_x : f32, pos_y : f32) {
        self.planet3_pos = Vec2::new(pos_x, pos_y);
    }

    pub fn update_p4_pos(&mut self, pos_x : f32, pos_y : f32) {
        self.planet4_pos = Vec2::new(pos_x, pos_y);
    }

    pub fn update_p5_pos(&mut self, pos_x : f32, pos_y : f32) {
        self.planet5_pos = Vec2::new(pos_x, pos_y);
    }
}
