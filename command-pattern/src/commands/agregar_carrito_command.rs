use crate::receiver::carrito_compras::CarritoCompras;
use crate::models::prenda::Prenda;
use crate::commands::command::Command;

use std::rc::Rc;
use std::cell::RefCell;

pub struct AgregarPrendaCommand {
    carrito: Rc<RefCell<CarritoCompras>>,
    prenda: Prenda,
}

impl AgregarPrendaCommand {

    // Constructor
    pub fn new(carrito: Rc<RefCell<CarritoCompras>>, prenda: Prenda) -> Self {
        AgregarPrendaCommand{carrito, prenda}
    }
}

impl Command for AgregarPrendaCommand {
    fn execute(&mut self) -> bool {
        self.carrito.borrow_mut().agregar_prenda(self.prenda.clone())
    }

    fn undo(&mut self) {
        self.carrito.borrow_mut().remover_prenda(self.prenda.clone())
    }
}
