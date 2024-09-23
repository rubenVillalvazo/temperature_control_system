# Sistema de Control de Temperatura - Proyecto de Aprendizaje de Patrones de Diseño

Este proyecto es un sistema simulado de control de temperatura enfocado en aplicar patrones de diseño en el contexto de sistemas embebidos. Es un proyecto sencillo pero práctico para aprender conceptos básicos de sistemas embebidos y cómo integrar patrones de diseño como Singleton, Factory Method, Observer y Strategy en Rust.

## Objetivo

Crear un sistema que controle la temperatura de una habitación usando sensores y actuadores (ventilador y calefactor). El sistema simula decisiones de encender o apagar dispositivos según los datos proporcionados por diferentes tipos de sensores.

### Patrones de diseño aplicados

- **Singleton:** Controlador de temperatura.
- **Factory Method:** Creación de sensores de temperatura (alta y baja precisión).
- **Observer:** Actuadores que reaccionan a cambios en la temperatura (ventilador y calefactor).
- **Strategy:** Diferentes estrategias de control de temperatura (control estricto y flexible).

## Estructura del Proyecto

1. src/: Código fuente en Rust.
2. README.md: Documentación del proyecto.
3. Cargo.toml: Configuración del proyecto Rust.

## Checklist de Progreso

### Fase 1: Implementación del Patrón Singleton

- [x] Crear el controlador de temperatura como Singleton.
- [x] Integrar simulación básica de temperatura.

### Fase 2: Implementación del Patrón Factory Method

- [x] Implementar diferentes tipos de sensores (alta y baja precisión).
- [x] Integrar los sensores con el controlador de temperatura.

### Fase 3: Implementación del Patrón Observer

- [ ] Crear los actuadores (ventilador y calefactor).
- [ ] Configurar los actuadores como observadores del controlador de temperatura.

### Fase 4: Implementación del Patrón Strategy

- [ ] Implementar diferentes estrategias de control de temperatura.
- [ ] Integrar las estrategias con el sistema principal.

### Fase 5: Simulación y pruebas

- [ ] Simular cambios de temperatura.
- [ ] Probar el sistema con diferentes sensores y estrategias de control.

## Cómo ejecutar el proyecto

1.

```bash
git clone https://github.com/rubenVillalvazo/temperature_control_system.git
cd temperature_control_system
```

2.

```bash
cargo run
```

## Contribuciones

Este es un proyecto abierto y en progreso. Se aceptan contribuciones y sugerencias.
