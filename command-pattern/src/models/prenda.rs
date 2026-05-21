#[derive(Clone, Debug, PartialEq)]
pub struct Prenda {
    pub nombre: String,
    pub cantidad: u32,
}

impl Prenda {

    // Equivalente al constructor en Java
    pub fn new(nombre: &str, cantidad: u32) -> Self {
        Prenda {
            nombre: nombre.to_string(),
            cantidad
        }
    }
}
