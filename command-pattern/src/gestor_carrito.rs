use crate::commands::command::Command;
pub struct GestorCarrito {
    // La pila donde van los comandos después de su ejecución
    historial: Vec<Box<dyn Command>>,
}

impl GestorCarrito {

    // Constructor
    pub fn new() -> Self {
        GestorCarrito {
            historial: Vec::new(),
        }
    }

    // El invocador ordena la ejecución del comando
    pub fn procesar_comando(&mut self, mut comando: Box<dyn Command>) {

        // Si la ejecución devuelve true, entonces se agrega al vector "carrito de compras"
        if comando.execute() {
            self.historial.push(comando);
        }
    }

    // El invocador deshace la ejecución del último comando guardado
    pub fn deshacer_ultima_accion(&mut self) {
        if let Some(mut ultimo_comando) = self.historial.pop() {
            ultimo_comando.undo(); // Ejecuta la acción inversa de execute()
        } else {
            println!("> No hay adiciones recientes. Imposible deshacer.");
        }
    }

}

