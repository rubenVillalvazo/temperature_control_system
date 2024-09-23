mod prelude;
mod sensor;
mod temperature_controller;

use crate::prelude::*;

fn main() {
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
}
