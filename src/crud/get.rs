use crate::{common::common::show_assignment, interface::task_inteface::RegisterTask};

pub fn task_list(task_list: Vec<RegisterTask>) {
    if task_list.len() == 0 {
        println!("No hay tareas registradas!!!")
    } else {
        for value in task_list {
            println!("--------------------------------------- \n");
            println!("Id: {}", value.id);
            println!("Descripcion de tarea: {}", value.description);
            println!("Estado: {}", value.status_progress);
            println!("Fecha de creacion: {}", value.created_at);
            println!("Fecha de modificacion: {:?}", value.updated_at);
            println!("--------------------------------------- ");
            println!("\n");
        }
    }
}

pub fn get_task_by_options(task_list: Vec<RegisterTask>, status_progress: String) {
    for task_done in task_list {
        if task_done.status_progress == status_progress {
            show_assignment(&task_done);
        }
    }
}
