use crate::prelude::*;

pub struct Clima {
    pub temperature: f64,
    pub is_temp_increasing: bool,
}

impl Clima {
    pub fn new() -> Clima {
        let mut rng = rand::thread_rng();
        let mut initial_temperature = rng.gen_range(-10.0..40.0);

        let mut is_increasing = initial_temperature > 12.5;

        Clima {
            temperature: initial_temperature,
            is_temp_increasing: is_increasing,
        }
    }
}
