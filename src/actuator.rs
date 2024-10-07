// Trait ue define la interfaz común para los actuadores (observadores).
// Todos los actuadores deben implementar el método `update()`, que será llamado cuando cambie la temperatura
pub trait Actuador {
    fn update(&self, temperature: f64);
}

// Implementación del acturador Ventilador.
// El ventilador se enciende si la temperatura supera los 25°C.
pub struct Fan;

impl Actuador for Fan {
    fn update(&self, temperature: f64) {
        if temperature > 25.0 {
            println!(
                "Ventilador encendido: La temperatura es {}°C. Enfriando...",
                temperature
            )
        } else {
            println!("Ventilador apagado.")
        }
    }
}

// Implementación del actuador Calefactor.
// El Calefactor se enciende si la temperatura es menor a a 18°C.

pub struct Heater;

impl Actuador for Heater {
    fn update(&self, temperature: f64) {
        if temperature < 18.0 {
            println!(
                "Calefactor encendido: La temperatura es {}°C. Calentando...",
                temperature
            );
        } else {
            println!("Calefactor apagado.");
        }
    }
}
