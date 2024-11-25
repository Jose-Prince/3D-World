use crate::Key;

pub struct Autopilot {
    pub active: bool,
    pub simulated_keys: Vec<Key>, // Lista de teclas simuladas
    pub progress: f32,           // Progreso del piloto automático (0.0 a 1.0)
    pub distance_traveled: f32,  // Distancia acumulada en el avance
}

impl Autopilot {
    pub fn new() -> Self {
        Self {
            active: false,
            simulated_keys: vec![],
            progress: 0.0,
            distance_traveled: 0.0,
        }
    }

    pub fn start(&mut self) {
        self.active = true;
        self.simulated_keys = vec![Key::A, Key::W]; // Simula giro y avance
        self.progress = 0.0;
        self.distance_traveled = 0.0;
    }

    pub fn stop(&mut self) {
        self.active = false;
        self.simulated_keys.clear();
        self.progress = 0.0;
        self.distance_traveled = 0.0;
    }
}

