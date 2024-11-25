//Barrel Roll
//The most important struct in the project
pub struct BarrelRoll {
    pub active: bool,
    pub progress: f32,
    pub rotation_y: f32,
}

impl BarrelRoll {
    pub fn update_rotation(&mut self, new_rotation : f32) {
        self.rotation_y = new_rotation;
    }
}
