mod actuator;
mod prelude;
mod sensor;
mod temperature_controller;

use crate::prelude::*;

fn main() {
    println!("Sistem de control de temperatura inicado.");

    // Obtenemos el controlador de temperatura.
    let controller = TemperatureController::instance();

    // Creamos los actuadores (Ventilador y Calefactor).
    let fan = Box::new(Fan);
    let heater = Box::new(Heater);

    {
        let mut locked_controller = controller.lock().unwrap();
        locked_controller.add_observer(fan);
        locked_controller.add_observer(heater);
    }

    // Simulamos un cambio de temperatura.
    {
        let mut locked_controller = controller.lock().unwrap();
        locked_controller.set_temperature(27.0); // Esto debería activar el ventilador.
        locked_controller.set_temperature(16.0); // Esto debería activar el calefactor.
    }

    /*
    set_temperature_controller();

    // Creando sensores a través de la fábrica
    let high_precision_sensor = SensorImplementation::create_sensor("high");
    let low_precision_sensor = SensorImplementation::create_sensor("low");

    // Leyendo la temperatura de cada sensor
    println!(
        "Alta precisión: {} °C",
        high_precision_sensor.read_temperature()
    );
    println!(
        "Baja precisión: {} °C",
        low_precision_sensor.read_temperature()
    );
    */
}
