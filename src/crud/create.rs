use crate::interface::task_inteface::{RegisterTask, StatusTask};
use std::io::stdin;
use regex::Regex;

pub fn task_entry(id: i64) -> RegisterTask {
    let mut description: String = String::new();
    let mut option_status_progress: String = String::new(); 
    let mut status_progress: StatusTask = StatusTask::default();
    let mut date: String = String::new();
    let mut condition: u8 = 0;
    let mut day: u8 = 0;
    let mut month: u8 = 0;
    let mut age: u16 = 0;
    let mut input_day: String = String::new();
    let mut input_month: String = String::new();
    let mut input_age: String = String::new();


    println!("Ingrese la descripcion de la tarea");
    
    while condition == 0 {
        let input_description = stdin().read_line(&mut description);
        match input_description {
            Ok(_) =>  {
                condition = 1
            },
            Err(_) => {
                println!("Problemas al ingresar la descripcion, intente de nuevo...")
            }
        }
    }

    println!("\n 
    Ingrese el estatus de la tarea segun el menu \n
    1. No ah inciado \n
    2. En progreso \n");

    while condition == 1 {
        let input_status_progress= stdin()
            .read_line(&mut option_status_progress);

        match input_status_progress {
            Ok(_) => {
                let status_progress_int: u8 = option_status_progress.trim().parse().expect("No esta ingresando un digito valido intentelo de nuevo..");
            
                match status_progress_int {
                    1 => {
                        status_progress = StatusTask::NotStarter;
                        condition = 2;
                    },
                    2 => {
                        status_progress = StatusTask::InProgress;
                        condition = 2;
                    },
                    _ => {
                        println!("Ingrese un digito segun el menu, intentelo de nuevo::")
                    }
                }
            
            },
            Err(_) => {
                println!("Error al ingresar estatus intentelo de nuevo")
            }
        }

    }
    

    println!("Ingrese fecha de creacion:");
    println!("Dia 1-31");

    while condition == 2 {
        let int_day = stdin().read_line(&mut input_day);

        match int_day {
            Ok(_) => {
                day = input_day.trim().parse().expect("Error: Dato que ingreso no es un digito");
            },
            Err(_) => {
                println!("No esta ingresando un digito valido intentelo de nuevo")
            }
        }
       

        if day < 1 || day > 31 {
            println!("Ingrese valores entre el 1 y 31")
        } else {
            condition = 3
        }      
    }

    println!("Mes 1-12");

    while condition == 3 {
        stdin().read_line(&mut input_month).expect("Error al ingresar mes intentelo de nuevo");
        month = input_month.trim().parse().expect("Dato que ingreso no es un digito intentelo de nuevo");

        if month < 1 || month > 31 {
            println!("Ingrese valores entre el 1 y 12")
        } else {
            condition = 4
        }
    }

    println!("Año 1900-2025");

    while condition == 4 {
        stdin().read_line(&mut input_age).expect("Error al ingresar dia intentelo de nuevo");
        age = input_age.trim().parse().expect("Dato que ingreso no es un digito intentelo de nuevo");

        if age < 1990 || age > 2024 {
            println!("Ingrese valores entre el 1 y 31")
        } else {
            condition = 5
        }
        
    }

    date = format!("{}-{}-{}",age,month,day);


    let task: RegisterTask = RegisterTask {
        id: id,
        description: description.trim().to_owned().clone(),
        status_progress: format!("{:?}",status_progress),
        created_at: date,
        updated_at: None 
    };


    task

    }



pub fn create_register_task(register: Vec<RegisterTask>) {

} 