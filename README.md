# Patrones de diseño: Command y Memento

Implementación de patrones de diseño GoF (Command y Memento )




# Misión Compras - Patron Command

Una aplicación que simula el carrito de compras del negocio de una tienda de ropa, en la cual se implementa el patrón de diseño Command para demostrar su funcionalidad. La implementación demuestra como Command es capaz de manejar la funcionalidad de undo(), "deshacer" en español mediante el uso de una pila de comandos, así como el desacoplamiento del módulo que emite una solicitud de quien realiza esa solicitud mediante el uso de un comando concreto y un receptor.

\---

# Bloc de Notas - Patron Memento (GoF)

Un bloc de notas de escritorio construido en Rust con interfaz grafica via Slint, que implementa el patron de diseño Memento del Gang of Four de forma estricta para gestionar deshacer/rehacer.

\---

## Requisitos Previos (Windows)

### 1\. Rust y Cargo

Descarga e instala el instalador oficial desde https://www.rust-lang.org/tools/install

Durante la instalacion te preguntara si quieres instalar las Build Tools de Visual Studio. Acepta esa opcion, es necesaria para compilar en Windows.

Si ya tienes Rust instalado pero no las Build Tools, puedes instalarlas por separado desde:
https://visualstudio.microsoft.com/visual-cpp-build-tools/

Selecciona el componente "Desarrollo para el escritorio con C++".
Nota: aunque el componente este destinado para C++ es necesario descargarlo para el correcto funcionamiento de los códigos de este repositorio.

Verifica que Rust quedo bien instalado abriendo una terminal nueva y ejecutando:

```bash
rustc --version
cargo --version
```

### 2\. Extensiones de VS Code para ambos ejemplos de implementación

* **rust-analyzer** — autocompletado, errores en tiempo real y navegacion de codigo Rust (Command y Memento)
* **Slint** — resaltado de sintaxis para los archivos .slint (Memento)

Las puedes buscar directamente en el panel de extensiones de VS Code (Ctrl+Shift+X).

\---

## Estructura del Proyecto: Command

```
command-pattern/
├── src/
│   ├── commands/
│   │   ├── agregar_carrito_command.rs
│   │   ├── command.rs
│   │   └── mod.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── prenda.rs
│   ├── receiver/
│   │   ├── carrito_compras.rs
│   │   └── mod.rs
│   ├── gestor_carrito.rs
│   └── main.rs
├── Cargo.lock
└── Cargo.toml
```

\---


## Estructura del Proyecto: Memento

```
memento-pattern/
├── src/
│   ├── main.rs
│   └── ui/
│       └── notepad.slint
├── Cargo.lock
├── Cargo.toml
```

\---

## Como Ejecutar ambos ejemplos

Abre la terminal en VS Code dentro de la carpeta del proyecto y ejecuta:

```bash
cargo run
```

En el caso del patrón Memento la primera compilacion tardara unos minutos mientras se descargan y compilan las dependencias (principalmente Slint). Las siguientes ejecuciones seran mucho mas rapidas.

\---

## Funcionalidades (Command)

|Accion                                                                 |Metodo o clase de origen |
|-----------------------------------------------------------------------|-------------------------|
|Añadir una nueva prenda al carrito de compras                          |execute() - método       |
|Ver el estado del carrito de compras y la lista de prendas descartadas |receptor - clase         |
|Eliminar última prenda agregada (undo)                                 |undo() - método          |
|Recuperar una prenda de la lista de prendas descartadas                |execute() - método       |

Los cambios se ven reflejados en la consola.

\---

## Funcionalidades (Memento)

|Accion  |Metodo                          |
|--------|--------------------------------|
|Escribir|Directamente en el area de texto|
|Deshacer|Boton "Deshacer" o Ctrl+Z       |
|Rehacer |Boton "Rehacer" o Ctrl+Y        |

Los botones se activan y desactivan automaticamente segun el estado del historial.

\---

## Patron de Diseño: Command

La logica de execute y undo se basan en los tres libros de la bibliografía.

```
Cliente ---> Invocador ---> Comando Concreto ---> Receptor
```

## Patron de Diseño: Memento (GoF)

La logica de deshacer/rehacer sigue fielmente el diagrama del Gang of Four:

```
Originator ---crea---> Memento <---o Caretaker
```

### Roles (Command)

|Clase           |Responsabilidad                                                                     |
|----------------|------------------------------------------------------------------------------------|
|Client          |Crea los comandos concretos                                                         |
|Invoker         |Ordena la ejecución de los comandos                                                 |
|Command         |Interfaz con métodos execute() y undo()                                             |
|ConcreteCommand |Implementa la interfaz Command                                                      |
|Receiver        |Clase de más bajo nivel que contiene los métodos orquestados por el ConcreteCommand |

### Roles (Memento)

|Clase     |Responsabilidad                                           |
|----------|----------------------------------------------------------|
|Memento   |Almacena el estado. Expone get\_state() y set\_state()    |
|Originator|Crea mementos y se restaura desde ellos                   |
|Caretaker |Custodia un Memento sin inspeccionarlo ni modificarlo     |
|History   |Gestiona dos pilas de Caretaker: undo\_stack y redo\_stack|

### Flujo (Command)

**Client:**
Crea el comando concreto y se lo entrega al Invoker.

**Invoker:**
Ordena la ejecución de los comandos concretos.

**ConcreteCommand:**
Llama a los métodos del Receiver para cumplir con la funcionalidad asignada.

**Receiver:**
Ejecuta los métodos que se encargan de realizar la funcionalidad.

### Flujo (Memento)

**Escribir texto:**
El Originator crea un Memento del estado previo, un Caretaker lo guarda en undo\_stack, y el redo\_stack se limpia.

**Ctrl+Z (Undo):**
Se extrae el ultimo Caretaker de undo\_stack, el estado actual se guarda en redo\_stack, y el Originator se restaura al estado anterior.

**Ctrl+Y (Redo):**
Se extrae el ultimo Caretaker de redo\_stack, el estado actual se guarda en undo\_stack, y el Originator avanza al estado siguiente.

\---

## Dependencias (Command)

```toml
\\\[dependencies]
<NINGUNA>
```

\---

## Dependencias (Memento)

```toml
\\\[dependencies]
slint = "1.16.0"

\\\[build-dependencies]
slint-build = "1.16.0"
```

\---

## Tecnologias

* Rust (Command y Memento)
* Slint (UI declarativa para aplicaciones nativas) para el patrón Memento
