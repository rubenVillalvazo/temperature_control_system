use crate::prelude::*;

// Trait Sensor que define la interfaz común para todos los sensores.
// Todos los sensores deben implementar el método `read_temperature()`.
pub trait Sensor {
    fn read_temperature(&self) -> f64;
}

// Sensor de alta precisión.
// Implementa la interfaz `Sensor` y simula una lectura de temperatura con un rango más estrecho.
pub struct HighPrecisionSensor;

/// Sensor de baja precisión.
/// Implementa la interfaz `Sensor` y simula una lectura de temperatura con un rango más amplio.

pub struct LowPrecisionSensor;

impl Sensor for HighPrecisionSensor {
    fn read_temperature(&self) -> f64 {
        // Simulación de un sensor de alta precisión, generando un número aleatorio entre 19.5°C y 20.5°C.
        let mut rng = rand::thread_rng();
        rng.gen_range(19.5..20.5)
    }
}

impl Sensor for LowPrecisionSensor {
    fn read_temperature(&self) -> f64 {
        // Simulación de un sensor de baja precisión, generando un número aleatorio entre 18.0°C y 22.0°C.
        let mut rng = rand::thread_rng();
        rng.gen_range(18.0..22.0)
    }
}

// Fábrica de Sensores.
// Este struct se encarga de crear instancias de sensores según el tipo solicitado.
pub struct SensorImplementation {}

impl SensorImplementation {
    // Método que crea y devuelve una instancia de un sensor según el tipo.
    // Se utilizan `Box<dyn Sensor>` para manejar polimorfismo y retornar cualquier tipo que implemente `Sensor`.
    pub fn create_sensor(sensor_type: &str) -> Box<dyn Sensor> {
        match sensor_type {
            // Si el tipo es "high", crea un sensor de alta precisión.
            "high" => Box::new(HighPrecisionSensor),
            // Si el tipo es "low", crea un sensor de baja precisión.
            "low" => Box::new(LowPrecisionSensor),
            // Si se pasa un tipo desconocido, se genera un pánico en el programa.
            _ => panic!("Uknow sensor type!"),
        }
    }
}
