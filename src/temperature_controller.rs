use crate::prelude::*;

// Estructura que representa el controlador de temperatura.
// Se utilizará como un Singleton para asegurarse de que solo exista una instancia de esta estructura.
struct TemperatureController {
    temperature: f64, // Campo para almacenar la temperatura actual.
    observers: Vec<Box<dyn Actuador>>,
}

impl TemperatureController {
    // Método para obtener la única instancia de `TemperatureController`.
    // Este método implementa el patrón Singleton.
    fn instance() -> Arc<Mutex<Self>> {
        // Usamos una variable estática mutable para almacenar la única instancia de `TemperatureController`.
        // La anotación `unsafe` es necesaria porque las variables estáticas mutables requieren cuidado
        // en cuanto a condiciones de carrera (race conditions) en programas concurrentes.
        static mut CONTROLLER: Option<Arc<Mutex<TemperatureController>>> = None;

        unsafe {
            // Verificamos si `CONTROLLER` ya tiene un valor. Si no lo tiene, creamos una nueva instancia.
            CONTROLLER
                .get_or_insert_with(|| {
                    // Si no existe aún, creamos una nueva instancia de `TemperatureController` con una temperatura inicial.
                    Arc::new(Mutex::new(TemperatureController {
                        temperature: 20.0,
                        observers: Vec::new(),
                    }))
                    // Temperatura inicial
                })
                // Retornamos una copia de la referencia a la instancia.
                .clone()
        }
    }

    // Añadir un nuevo observador (Actuador) a la lista.
    fn add_observer(&mut self, observer: Box<dyn Actuador>) {
        self.observers.push(observer);
    }

    // Notificar a los observadores cuando cambie la temperatura.
    fn notify_observers(&self) {
        for observer in &self.observers {
            observer.update(self.temperature);
        }
    }

    // Método para cambiar la temperatura.
    // Este método será llamado cuando queramos modificar la temperatura en el controlador.
    fn set_temperature(&mut self, new_temp: f64) {
        self.temperature = new_temp; // Actualizamos el valor de la temperatura.
        self.notify_observers();
    }

    // Método para obtener la temperatura actual.
    // Esto nos permite consultar el valor de la temperatura en cualquier momento.
    fn get_temperature(&self) -> f64 {
        self.temperature // Retornamos el valor actual de la temperatura.
    }
}

// Función auxiliar que se encarga de interactuar con el controlador de temperatura.
pub fn set_temperature_controller() {
    // Obtenemos la instancia Singleton del controlador de temperatura.
    let controller = TemperatureController::instance();

    // Bloqueamos el acceso al controlador para poder modificar la temperatura de manera segura.
    {
        let mut locked_controller = controller.lock().unwrap(); // Usamos `lock()` para obtener acceso mutable.
        locked_controller.set_temperature(25.0); // Simulamos un cambio de temperatura.
    }

    // Bloqueamos nuevamente, esta vez para leer la temperatura.
    {
        let locked_controller = controller.lock().unwrap(); // Usamos `lock()` para obtener acceso inmutable.
        println!(
            "Temperatura actual: {} °C",
            locked_controller.get_temperature() // Mostramos la temperatura actual.
        );
    }
}
