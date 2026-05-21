# patrones-command-y-memento

Implementación de patrones de diseño GoF (Command y Memento )









# Bloc de Notas - Patron Memento (GoF)

Un bloc de notas de escritorio construido en Rust con interfaz grafica via Slint, que implementa el patron de diseno Memento del Gang of Four de forma estricta para gestionar deshacer/rehacer.

\---

## Requisitos Previos (Windows)

### 1\. Rust y Cargo

Descarga e instala el instalador oficial desde https://www.rust-lang.org/tools/install

Durante la instalacion te preguntara si quieres instalar las Build Tools de Visual Studio. Acepta esa opcion, es necesaria para compilar en Windows.

Si ya tienes Rust instalado pero no las Build Tools, puedes instalarlas por separado desde:
https://visualstudio.microsoft.com/visual-cpp-build-tools/

Selecciona el componente "Desarrollo para el escritorio con C++".

Verifica que Rust quedo bien instalado abriendo una terminal nueva y ejecutando:

```bash
rustc --version
cargo --version
```

### 2\. Extensiones de VS Code (recomendadas)

* **rust-analyzer** — autocompletado, errores en tiempo real y navegacion de codigo Rust
* **Slint** — resaltado de sintaxis para los archivos .slint

Las puedes buscar directamente en el panel de extensiones de VS Code (Ctrl+Shift+X).

\---

## Estructura del Proyecto

```
memento-pattern/
├── src/
│   ├── main.rs
│   └── ui/
│       └── notepad.slint
├── Cargo.toml
└── build.rs
```

\---

## Como Ejecutar

Abre la terminal en VS Code dentro de la carpeta del proyecto y ejecuta:

```bash
cargo run
```

La primera compilacion tardara unos minutos mientras se descargan y compilan las dependencias (principalmente Slint). Las siguientes ejecuciones seran mucho mas rapidas.

\---

## Funcionalidades

|Accion|Metodo|
|-|-|
|Escribir|Directamente en el area de texto|
|Deshacer|Boton "Deshacer" o Ctrl+Z|
|Rehacer|Boton "Rehacer" o Ctrl+Y|

Los botones se activan y desactivan automaticamente segun el estado del historial.

\---

## Patron de Diseno: Memento (GoF)

La logica de deshacer/rehacer sigue fielmente el diagrama del Gang of Four:

```
Originator ---crea---> Memento <---o Caretaker
```

### Roles

|Clase|Responsabilidad|
|-|-|
|Memento|Almacena el estado. Expone get\_state() y set\_state()|
|Originator|Crea mementos y se restaura desde ellos|
|Caretaker|Custodia un Memento sin inspeccionarlo ni modificarlo|
|History|Gestiona dos pilas de Caretaker: undo\_stack y redo\_stack|

### Flujo

**Escribir texto:**
El Originator crea un Memento del estado previo, un Caretaker lo guarda en undo\_stack, y el redo\_stack se limpia.

**Ctrl+Z (Undo):**
Se extrae el ultimo Caretaker de undo\_stack, el estado actual se guarda en redo\_stack, y el Originator se restaura al estado anterior.

**Ctrl+Y (Redo):**
Se extrae el ultimo Caretaker de redo\_stack, el estado actual se guarda en undo\_stack, y el Originator avanza al estado siguiente.

\---

## Dependencias

```toml
\\\[dependencies]
slint = "1.16.0"

\\\[build-dependencies]
slint-build = "1.16.0"
```

\---

## Tecnologias

* Rust
* Slint (UI declarativa para aplicaciones nativas)
* Patron Memento (GoF)



