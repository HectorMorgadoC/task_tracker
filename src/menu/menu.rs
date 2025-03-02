use std::{io::stdin, num::ParseIntError};


pub fn option_menu(digite: u8) {

    let digite_option: u8 = digite;
    match digite_option {
        1 => {

        },
        _ => {

        }
    }
}




pub fn main_menu() -> u8 {
    let mut menu: String = String::new();

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

    let result = stdin()
    .read_line(&mut menu);

    match result {
        Ok(_) => {
            println!("---------> Ingreso de valor")
        },
        Err(_) => {
            println!("Error al ingresar digito")
        }
    }

    let value_output: Result<u8,ParseIntError> = menu.trim().parse();

    match value_output {
        Ok(value) => {
            value
        },
        Err(_) => {
            println!("Dato ingresado no es un digito:-- ");
            0
        }
    }

}