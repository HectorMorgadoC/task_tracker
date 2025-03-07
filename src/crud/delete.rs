use crate::{common::common::delete_task_by_id, interface::task_inteface::RegisterTask};

pub fn delete_task(task_list: Vec<RegisterTask> ) -> Vec<RegisterTask> {
    
    println!("Inserte la id de la tarea");
    delete_task_by_id(&task_list)

} 