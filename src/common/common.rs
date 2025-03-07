use crate::interface::task_inteface::RegisterTask;
use std::io::{stdin, Error};
use std::num::ParseIntError;

pub fn task_creation_date_entry(message_reference: &str) -> String {
    println!("{}", message_reference);
    let day: u16 = convert_date_format(1, 31, "day");
    let month: u16 = convert_date_format(1, 12, "month");
    let age: u16 = convert_date_format(1990, 2025, "age");

    let date: String = format!("{}-{}-{}", age, month, day);

    date
}

pub fn validate_terminal_line_entry() -> String {
    let mut terminal_line_value: String = String::new();
    let mut condition: bool = false;
    let validate: Result<usize, Error> = stdin().read_line(&mut terminal_line_value);

    while !condition {
        match validate {
            Ok(_) => {
                condition = true;
            }
            Err(_) => {
                println!("Error de ingreso de dato en la terminal")
            }
        }
    }
    terminal_line_value
}

pub fn convert_date_format(min: u16, max: u16, message: &str) -> u16 {
    let mut condition: bool = false;
    let mut date_number: u16 = 0;

    println!("{} {}-{}", message, min, max);

    while !condition {
        let convert_data_int: Result<u16, ParseIntError> =
            validate_terminal_line_entry().trim().parse::<u16>();

        match convert_data_int {
            Ok(_value) => {
                if _value < min || _value > max {
                    println!("Ingrese valores entre el {} y {}", min, max)
                } else {
                    date_number = _value;
                    condition = true;
                }
            }
            Err(_err) => {
                println!("Error: Dato que ingreso no es un digito. \n {}", _err)
            }
        }
    }
    date_number
}

pub fn terminal_line_value_int64() -> u64 {
    let mut value_int_format: u64 = 0;
    let mut condition: bool = false;

    while !condition {
        let parser_u64: Result<u64, ParseIntError> = validate_terminal_line_entry().trim().parse();
        match parser_u64 {
            Ok(value) => {
                value_int_format = value;
                condition = true;
            }
            Err(ref err) => {
                println!("Error: datos ingresados no son un numero entero: {}", err)
            }
        }
    }

    value_int_format
}

pub fn terminal_line_value_int8() -> u8 {
    let mut value_int_format: u8 = 0;
    let mut condition: bool = false;

    while !condition {
        let parser_u8: Result<u8, ParseIntError> = validate_terminal_line_entry().trim().parse();
        match parser_u8 {
            Ok(value) => {
                value_int_format = value;
                condition = true;
            }
            Err(ref err) => {
                println!("Error: datos ingresados no son un numero entero: {}", err)
            }
        }
    }

    value_int_format
}

pub fn search_task_by_id(task_list: &Vec<RegisterTask>) -> RegisterTask {
    let mut condition: bool = false;
    let mut task: RegisterTask = RegisterTask::default();

    while !condition {
        let id: u64 = terminal_line_value_int64();

        for value in task_list {
            if value.id == id {
                task = value.clone();
                condition = true;
            }
        }

        if !condition {
            println!("Id no existe: Ingrese id correcto")
        }
    }

    task
}

pub fn delete_task_by_id(task_list: &Vec<RegisterTask>) -> Vec<RegisterTask> {
    let mut condition: bool = false;
    let mut new_task_list: Vec<RegisterTask> = task_list.clone();

    while !condition {
        let id: u64 = terminal_line_value_int64();

        for (index, value) in task_list.iter().enumerate() {
            if value.id == id && question_yes_or_not("Desea eliminar el registro de la tarea?") {
                new_task_list.remove(index);
                condition = true;
            }
        }

        if !condition {
            println!("Id no existe: Ingrese id correcto")
        }
    }

    new_task_list
}

pub fn show_assignment(task: &RegisterTask) {
    println!("------------------------------");
    println!("Id: {}", task.id);
    println!("Descripcion de la tarea: {}", task.description);
    println!("Estado de progreso: {}", task.status_progress);
    println!("Fecha de creacion: {}", task.created_at);
    println!("------------------------------");
}

pub fn question_yes_or_not(reference_message: &str) -> bool {
    println!("{} \n 1 = si, 2 = no", reference_message);

    let number_option_yes_or_not: u8 = terminal_line_value_int8();
    let mut response: bool = false;

    if number_option_yes_or_not == 1 {
        response = true;
    }

    response
}
