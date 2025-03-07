use std::num::ParseIntError;
use crate::{crud::{create::create_task, delete::delete_task, get::task_list, update::update_task_record}, data::{data_management::data_management, file_management::write_file}};
use crate::common::common::validate_terminal_line_entry;

pub fn option_menu(digite: u8, ) {

    let data = data_management("data.json");
    let digite_option: u8 = digite;
    match digite_option {
        1 => {
            task_list(data);
        },
        2 => {
            write_file("data.json"
            ,create_task(data));
        },
        3 => {
            write_file("data.json"
            , update_task_record(data));
        },
        4 => {
            write_file("data.json"
            , delete_task(data));
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