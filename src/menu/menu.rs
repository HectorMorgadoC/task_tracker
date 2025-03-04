use std::num::ParseIntError;
use crate::{crud::{create::{create_task,validate_terminal_line_entry},get::task_list}, data::{data_management::data_management, file_management::write_file}};

pub fn option_menu(digite: u8, ) {

    let digite_option: u8 = digite;
    match digite_option {
        1 => {
            task_list(data_management("data.json"));
        },
        2 => {
            write_file("data.json",create_task(data_management("data.json")));
        },
        3 => {
            println!("No existe menu para ello")
        }
        _ => {

        }
    }
}

pub fn main_menu() -> u8 {
    
    println!("\n
    TASK LOGS \n
    1. listar todas las tareas \n
    2. Agregar una tarea \n
    3. Actualizar una tarea \n
    4. Eliminar una tarea \n
    5. Ver las tareas culminadas \n
    6. Ver las tareas pendientes \n
    7. Ver las tareas en curso \n
    ");

    let menu: String = validate_terminal_line_entry();
    let mut condition: bool = false;
    let mut menu_index: u8 = 0;

    while !condition {

        let value_output: Result<u8,ParseIntError> = menu.trim().parse();

        match value_output {
            Ok(value) => {
                menu_index = value;
                condition = true
            },
            Err(_) => {
                println!("Dato ingresado no es un digito:-- ");
            }
        }
    }
    menu_index
}