use nalgebra_glm::{Vec3, normalize, cross, magnitude};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3,  // Posición de la cámara en el espacio mundial
    pub center: Vec3, // Punto que la cámara está mirando
    pub up: Vec3,  // Vector hacia arriba
}

impl Camera {
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        // Convertir la dirección del rayo del espacio de la cámara al espacio del mundo
        let rotated = vector.x * right + vector.y * up - vector.z * forward;

        rotated
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // Calcular el vector desde el centro hasta el ojo (vector de radio) y medir la distancia
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        // Calcular yaw actual (rotación alrededor del eje Y)
        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        // Calcular pitch actual (rotación alrededor del eje X)
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Aplicar rotaciones delta
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        // Calcular la nueva posición del ojo
        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );

        self.eye = new_eye;
    }

    pub fn get_yaw(&self) -> f32 {
        let direction = self.center - self.eye;
        direction.z.atan2(direction.x)
    }

    pub fn get_pitch(&self) -> f32 {
        let direction = self.center - self.eye;
        let radius_xz = (direction.x * direction.x + direction.z *direction.z).sqrt();
        (-direction.y).atan2(radius_xz)
    }

    pub fn zoom (&mut self, delta: f32) {
        let direction = (self.eye - self.center).normalize();

        let new_eye = self.eye + direction * delta;

        //Set limit for camera zoom
        let min_distance = 0.1;
        if (new_eye - self.center).magnitude() > min_distance {
            self.eye = new_eye;
        }
    }
}
