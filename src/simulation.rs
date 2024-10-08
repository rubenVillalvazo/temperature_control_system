use crate::prelude::*;

pub struct Clima {
    pub temperature: f64,
    pub is_temp_increasing: bool,
}

impl Clima {
    pub fn new() -> Clima {
        let mut rng = rand::thread_rng();
        let initial_temperature = rng.gen_range(-10.0..40.0);

        let mut is_increasing = initial_temperature > 12.5;

        Clima {
            temperature: initial_temperature,
            is_temp_increasing: is_increasing,
        }
    }

    // Simular el cambio de temperatura
    pub fn actualizar(&mut self) {
        let mut rng = rand::thread_rng();
        let cambio = rng.gen_range(0.1..0.5); // Cambio pequeño en la temperatura

        if self.is_temp_increasing {
            self.temperature += cambio;
        } else {
            self.temperature -= cambio;
        }
    }
}
