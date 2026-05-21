mod receiver;
mod commands;
mod models;
mod gestor_carrito;
use gestor_carrito::GestorCarrito;

use crate::receiver::carrito_compras::CarritoCompras;
use crate::commands::agregar_carrito_command::AgregarPrendaCommand;
use crate::models::prenda::Prenda;

use std::io::{self, Write};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {

    // Instanciar al Receptor
    let carrito: Rc<RefCell<CarritoCompras>> = Rc::new(RefCell::new(CarritoCompras::new()));
    
    // Instanciar al Invocador
    let mut gestor: GestorCarrito = GestorCarrito::new();

    let catalogo_ropa: [&str; 5] = ["Camisa", "Playera", "Bufanda", "Vestido", "Sudadera"];
    let mut opcion: usize;

    println!("============== Misión Compras: Patrón Command ==============");

    loop {
        println!("\nMenú");
        println!("-----------------------------------------");
        println!("1. Añadir una nueva prenda al carrito");
        println!("2. Ver Carrito y prendas descartadas");
        println!("3. Eliminar última prenda agregada (Undo)");
        println!("4. Recuperar una prenda descartada");
        println!("5. Salir");
        println!("-----------------------------------------");
        print!("> Elige una opción: ");
        io::stdout().flush().unwrap();

        let mut input_opcion: String = String::new();
        io::stdin().read_line(&mut input_opcion).unwrap();
        
        opcion = input_opcion.trim().parse::<usize>().unwrap_or(0);

        match opcion {
            1 => {

                // --- AÑADIR NUEVA PRENDA AL CARRITO DE COMPRAS ---
                println!("\n--- Prendas disponibles ---");
                for (i, prenda) in catalogo_ropa.iter().enumerate() {
                    println!("  - {}. {}", i + 1, prenda);
                }
                print!("> Elige el número de prenda: ");
                io::stdout().flush().unwrap();

                let mut input_sel: String = String::new();
                io::stdin().read_line(&mut input_sel).unwrap();
                
                let num_seleccionado: usize = input_sel.trim().parse::<usize>().unwrap_or(0);

                // Validación: Número de prenda debe existir
                if num_seleccionado == 0 || num_seleccionado > catalogo_ropa.len() {
                    println!("[Error: La prenda seleccionada no existe]");
                    continue; // Regresa al menú principal de forma segura
                }
                let seleccion: usize = num_seleccionado - 1;

                print!("> Número de unidades: ");
                io::stdout().flush().unwrap();
                let mut input_cant: String = String::new();
                io::stdin().read_line(&mut input_cant).unwrap();
                
                let cantidad: u32 = input_cant.trim().parse::<u32>().unwrap_or(0);

                //Validación: Cantidad de prendas debe ser mayor a cero
                if cantidad == 0 {
                    println!("[Error: Cantidad inválida. El número de prendas debe ser mayor a 0]");
                    continue;
                }

                let nueva_prenda: Prenda = Prenda::new(catalogo_ropa[seleccion], cantidad);
                
                let carrito_asignado: Rc<RefCell<CarritoCompras>> = Rc::clone(&carrito);
                let comando: Box<AgregarPrendaCommand> = Box::new(AgregarPrendaCommand::new(carrito_asignado, nueva_prenda));
                
                gestor.procesar_comando(comando);
            }

            // -- MOSTRAR CARRITO DE COMPRAS ACTUAL Y LISTA DE PRENDAS DESCARTADAS --
            2 => {
                carrito.borrow().mostrar_estado();
            }

            // -- Uso de undo() para deshacer la última adición de una prenda --
            3 => {
                println!("\nDeshaciendo última adición...");
                gestor.deshacer_ultima_accion();
            }

            4 => {

                // --- RECUPERAR PRENDA DESCARTADA ---
                let prenda_a_rescatar: Option<Prenda> = {
                    let car: std::cell::Ref<'_, CarritoCompras> = carrito.borrow();
                    if car.get_prendas_descartadas().is_empty() {
                        println!("\n[Aviso] No hay prendas en la lista de descartados para recuperar.");
                        None
                    } else {

                        // Lista de prendas descartadas
                        println!("\n--- ¿Qué prenda deseas recuperar? ---");
                        for (i, p) in car.get_prendas_descartadas().iter().enumerate() {
                            println!("{}. {} (x{})", i + 1, p.nombre, p.cantidad);
                        }
                        print!("> Selecciona el índice de la prenda a rescatar: ");
                        io::stdout().flush().unwrap();

                        let mut input_idx: String = String::new();
                        io::stdin().read_line(&mut input_idx).unwrap();
                        
                        let idx_seleccionado: usize = input_idx.trim().parse::<usize>().unwrap_or(0);

                        // Validación: El índice debe pertenecer a la lista de descartados
                        if idx_seleccionado == 0 || idx_seleccionado > car.get_prendas_descartadas().len() {
                            println!("[ERROR] Índice de prenda descartada inválido.");
                            None
                        } else {
                            Some(car.get_prendas_descartadas()[idx_seleccionado - 1].clone())
                        }
                    }
                };

                if let Some(prenda) = prenda_a_rescatar {
                    let carrito_asignado: Rc<RefCell<CarritoCompras>> = Rc::clone(&carrito);
                    let comando: Box<AgregarPrendaCommand> = Box::new(AgregarPrendaCommand::new(carrito_asignado, prenda));
                    
                    gestor.procesar_comando(comando);
                }
            }

            // -- Salir de la aplicación --
            5 => {
                println!("Saliendo...");
                println!("¡Adios!");
                break;
            }

            // -- Default --
            _ => println!("[ERROR: Opción no válida]"),
        }
    }
}