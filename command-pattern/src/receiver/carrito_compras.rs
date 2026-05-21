use crate::models::prenda::Prenda;
pub struct CarritoCompras {
    carrito_actual: Vec<Prenda>,
    prendas_descartadas: Vec<Prenda>,
}

impl CarritoCompras {

    // Constructor
    pub fn new() -> Self {
        CarritoCompras {
            carrito_actual: Vec::new(),
            prendas_descartadas: Vec::new(),
        }
    }

    /// OPERACIONES DEL RECEPTOR

    // AGREGAR UNA PRENDA AL CARRITO DE COMPRAS
    pub fn agregar_prenda(&mut self, prenda: Prenda) -> bool{

        // Busca en la lista de "prendas_descartadas" si la prenda seleccionada ya existe
        if let Some(pos) = self.prendas_descartadas.iter().position(|p: &Prenda| p == &prenda) {

            // si existe entonces se remueve de "prendas_descartadas"
            self.prendas_descartadas.remove(pos);
        }

        // Añadir la prenda al carrito de compras
        self.carrito_actual.push(prenda.clone());
        println!("Se agregaron {} unidad(es) de {}.", prenda.cantidad, prenda.nombre);
        return true;
    }

    // REMOVER UNA PRENDA DEL CARRITO DE COMPRAS
    pub fn remover_prenda(&mut self, prenda: Prenda) {

        // Busca en la lista de "carrito_actual" si la prenda existe
        if let Some(pos) = self.carrito_actual.iter().position(|p: &Prenda| p == &prenda) {
            
            // si existe entonces la quitamos del "carrito de compras"
            let p: Prenda = self.carrito_actual.remove(pos);

            // almacenamos la prenda removida en la lista de prendas descartadas
            self.prendas_descartadas.push(p);
            println!("Se removieron {} {} a la lista de prendas descartadas.", prenda.cantidad, prenda.nombre);
        }
    }

    // OBTENER LA LISTA DE PRENDAS DESCARTADAS
    pub fn get_prendas_descartadas(&self) -> &Vec<Prenda> {
        &self.prendas_descartadas
    }

    // MOSTRAR CARRITO DE COMPRAS Y PRENDAS DESCARTADAS
    pub fn mostrar_estado(&self) {
        println!("\n===============================================");
        println!("Tu carrito actual:");
        if self.carrito_actual.is_empty() {
            println!("  > El carrito de compras está vacío");
        } else {
            for prenda in &self.carrito_actual {
                println!("  - {} (Cantidad: {})", prenda.nombre, prenda.cantidad);
            }
        }

        println!("\nPrendas descartadas (aún puedes recuperarlas)");
        if self.prendas_descartadas.is_empty() {
            println!("  > No tienes prendas descartadas aún");
        } else {
            for (i, prenda) in self.prendas_descartadas.iter().enumerate() {
                println!("  {}. {} (Cantidad: {}) ", i + 1, prenda.nombre, prenda.cantidad);
            }
        }
        println!("\n===============================================");
    }
}
