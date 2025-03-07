use crate::{
    common::common::{question_yes_or_not, terminal_line_value_int8},
    crud::{
        create::create_task,
        delete::delete_task,
        get::{get_task_by_options, task_list},
        update::update_task_record,
    },
    data::{data_management::data_management, file_management::write_file},
};

pub fn option_menu(path: &str) {
    let mut condition: bool = false;

    while !condition {
        let data = data_management(path);
        let digite_option: u8 = main_menu();
        match digite_option {
            1 => {
                task_list(data);
            }
            2 => {
                write_file(path, create_task(data));
            }
            3 => {
                write_file(path, update_task_record(data));
            }
            4 => {
                write_file(path, delete_task(data));
            }
            5 => {
                get_task_by_options(data, "Culminado".to_owned());
            }
            6 => {
                get_task_by_options(data, "EnProgreso".to_owned());
            }
            7 => {
                get_task_by_options(data, "SinIniciar".to_owned());
            }
            _ => {
                println!("El digito no es una opcion:")
            }
        }

        if !question_yes_or_not("Desea continuar?") {
            condition = true
        }
    }
}

pub fn main_menu() -> u8 {
    println!(
        "\n
    TASK LOGS \n
    1. listar todas las tareas \n
    2. Agregar una tarea \n
    3. Actualizar una tarea \n
    4. Eliminar una tarea \n
    5. Ver las tareas culminadas \n
    6. Ver las tareas pendientes \n
    7. Ver las tareas en curso \n
    "
    );

    let menu_index: u8 = terminal_line_value_int8();

    menu_index
}
