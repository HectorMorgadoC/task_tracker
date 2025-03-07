use crate::interface::task_inteface::{RegisterTask, StatusTask};
use crate::common::common::{terminal_line_value_int8, question_yes_or_not, search_task_by_id, show_assignment, task_creation_date_entry, validate_terminal_line_entry};

pub fn update_task_record(task_list:Vec<RegisterTask>) -> Vec<RegisterTask> {
    println!("Inserte la id de la tarea");
    
    let mut task: RegisterTask = search_task_by_id(&task_list);

    show_assignment(&task);

    if question_yes_or_not("Desea modificar la descripcion de tarea?") {
        println!("Ingrese nueva de descripcion de la tarea:");
        let description: String = validate_terminal_line_entry();
        task.description = description;
    }

    if question_yes_or_not("Desea modificar el estado de la tarea?") {
        println!("\n 
        Ingrese el estado de la tarea segun la lista \n
        1. No ah inciado \n
        2. En progreso \n
        3. Terminado");

        let mut status_progress: StatusTask = StatusTask::default();
        let mut condition: bool = false;

        let number_status_progress: u8 = terminal_line_value_int8();

        while !condition {
            match number_status_progress {
                1 => {
                    status_progress = StatusTask::NotStarter;
                    condition = true;
                },
                2 => {
                    status_progress = StatusTask::InProgress;
                    condition = true;
                },
                3 => {
                    status_progress = StatusTask::Done;
                    condition = true;
                },
                _ => {
                    println!("Error digito incorrecto: Ingrese un digito con el menu correcto")
                }
            }
        }

        task.status_progress = format!("{:?}",status_progress);
    }

    if question_yes_or_not("Desea modificar la fecha de creacion") {
        let date: String = task_creation_date_entry("Ingrese fecha de creacion");

        task.created_at = date;
    }

    let date_update: String = task_creation_date_entry("Ingrese fecha de modificacion");
    task.updated_at = Some(date_update);


    update_task(&task,&task_list)

}

fn update_task(task: &RegisterTask,task_list: &Vec<RegisterTask>) -> Vec<RegisterTask> {

    let mut new_task_list: Vec<RegisterTask> = Vec::new();

    for value in task_list {
        if task.id == value.id {
            new_task_list.push(task.clone())
        } else {
            new_task_list.push(value.clone())
        }
        
    }

    new_task_list.clone()
} 