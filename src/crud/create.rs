use rand::Rng;
use crate::common::common::{task_creation_date_entry, validate_terminal_line_entry};
use crate::interface::task_inteface::{RegisterTask, StatusTask};
use std::num::ParseIntError;


pub fn task_entry(id: u64) -> RegisterTask {
    
    let date: String = task_creation_date_entry("Ingrese fecha de creacion"); 
    let description: String = task_description_entry();
    let status_progress: StatusTask = task_status_entry();

    let task: RegisterTask = RegisterTask {
        id: id,
        description: description.trim().to_owned(),
        status_progress: format!("{:?}",status_progress),
        created_at: date,
        updated_at: None 
    };

    task
}


pub fn create_task(task_list: Vec<RegisterTask> ) -> Vec<RegisterTask> {
    let mut new_list_task: Vec<RegisterTask> = task_list;
    let mut condition: bool = false;
    let mut random = rand::rng();
    let mut random_number: u64 = random.random_range(1..10000);


    if !new_list_task.len() <= 0 {
        while !condition {
            for value in &new_list_task {
                if value.id == random_number {
                    random_number = random.random_range(1..10000);
                } else {
                    condition = true
                }
            }
        }
    } 
    

    let task: RegisterTask = task_entry(random_number);

    new_list_task.push(task);

    new_list_task

}


fn task_description_entry() -> String {

    println!("Ingrese la descripcion de la tarea");
    
    let description: String = validate_terminal_line_entry();
    
    description
}

fn task_status_entry() -> StatusTask {

    let mut condition: bool = false;
    let mut status_progress: StatusTask = StatusTask::default(); 

    println!("\n 
    Ingrese el estado de la tarea segun la lista \n
    1. No ah inciado \n
    2. En progreso \n");

    while !condition {
        
        let status_progress_int: Result<u8,ParseIntError> = validate_terminal_line_entry().trim().parse::<u8>();
                
            match status_progress_int {
                Ok(1) => {
                    status_progress = StatusTask::NotStarter;
                    condition = true;
                },
                Ok(2) => {
                    status_progress = StatusTask::InProgress;
                    condition = true;
                },
                Err(_) => {
                    println!("Error: Ingrese un digito valido")
                },
                _ => {
                    println!("Ingrese un digito segun el menu")
                },
            }
            
    }   
    status_progress             
}


