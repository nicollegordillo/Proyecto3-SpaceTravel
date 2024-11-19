# Sistema Solar Interactivo en Rust 🌌🚀

Este proyecto es una simulación interactiva de un sistema solar renderizado en **Rust**, que utiliza **shaders personalizados** y un modelo de cámara controlable para ofrecer una experiencia visual única. El usuario puede explorar diferentes planetas, ajustar la cámara y observar los efectos visuales creados con shaders programados.

---

## Características Principales

- **Planetas con Órbitas Dinámicas**: Los planetas orbitan alrededor del Sol a velocidades específicas.
- **Shaders Personalizados**: Cada cuerpo celeste tiene un shader único que simula diferentes características visuales.
- **Cámara Controlable**: Movimiento libre y orbitación de la cámara usando teclado (movimiento 3D).
- **Zoom Dinámico**: Ajuste de la distancia entre la cámara y los planetas.
- **Enfoque en Planetas**: Cambio rápido del objetivo de la cámara hacia planetas específicos (instant warp animado).
- **Diferentes vistas**: Puede cambiar entre vista aérea y vista normal.

---

## Controles 🕹️

### **Movimiento de la Cámara**
**Teclado**:
- **A/D**: Mover la cámara hacia la izquierda/derecha.
- **Q/E**: Mover la cámara hacia arriba/abajo.
- **W/S**: Rotar la cámara hacia arriba/abajo.
- **Izquierda/Derecha**: Rotar la cámara alrededor del eje Y.
- **Arriba/Abajo**: Hacer zoom (acercar o alejar).
- **ESC**: Cerrar el programa.

### **Cambio de Objetivo (instant warp)**
- **1**: Centrar la cámara en la Tierra.
- **2**: Centrar la cámara en Marte.
- **3**: Centrar la cámara en Júpiter.
- **4**: Centrar la cámara en Venus.
- **5**: Centrar la cámara en Mercurio.

### **Modos de Vista**
- **B**: Activar vista cenital (vista de pájaro).
- **N**: Volver a la vista normal.

---

## Instrucciones de Instalación y Ejecución 🚀

1. **Instalar Rust**  
   Descarga e instala Rust.

2. **Clonar el Repositorio**  
   ```bash
   git clone https://github.com/nicollegordillo/Proyecto3-SpaceTravel.git
   cd Proyecto3-SpaceTravel
   ```
3. **Compilar y Ejecutar**
    ```bash
   cargo run
   ```
## Video de Demostración 🎥
https://youtu.be/qC4YBlNGlHg
