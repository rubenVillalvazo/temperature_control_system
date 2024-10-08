mod actuator;
mod prelude;
mod sensor;
mod simulation;
mod temperature_controller;

use crate::prelude::*;

fn main() {
    // Inicializamos el sistema con el clima
    let mut clima = Clima::new();

    // Loop principal
    loop {
        // Actualizamos el estado del clima
        clima.actualizar();
        println!("Temperatura actual: {:.1}°C", clima.temperature);

        // Simulamos un tiempo de espera entre iteraciones (e.g. 1 segundo)
        thread::sleep(Duration::from_secs(5));

        // Podríamos agregar alguna condición para salir del loop si se desea
    }
}
