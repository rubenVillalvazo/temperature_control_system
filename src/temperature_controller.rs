use crate::prelude::*;

struct TemperatureController {
    temperature: f64,
}

impl TemperatureController {
    fn instance() -> Arc<Mutex<Self>> {
        static mut CONTROLLER: Option<Arc<Mutex<TemperatureController>>> = None;
        unsafe {
            CONTROLLER
                .get_or_insert((|| {
                    Arc::new(Mutex::new(TemperatureController { temperature: 20.0 }))
                })())
                .clone()
        }
    }

    fn set_temperature(&mut self, new_temp: f64) {
        self.temperature = new_temp;
    }

    fn get_temperature(&self) -> f64 {
        self.temperature
    }
}

pub fn set_temperature_controller() {
    let controller = TemperatureController::instance();

    // Bloqueamos para mutar la temperatura
    {
        let mut locked_controller = controller.lock().unwrap();
        locked_controller.set_temperature(25.0); // Simulando un cambio de temperatura
    }

    // Leemos la nueva temperatura
    {
        let locked_controller = controller.lock().unwrap();
        println!(
            "Temperatura actual: {} °C",
            locked_controller.get_temperature()
        );
    }
}
