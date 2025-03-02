mod interface;
mod data;
mod menu;
mod crud;

use interface::task_inteface::RegisterTask;
use crate::data::data_management::data_management;
use crate::menu::menu::main_menu;
use crate::crud::create::task_entry;


fn main() {
    println!("{}",main_menu());

    //let data_task: Vec<RegisterTask> = data_management("src/data.json");

    
    println!("{:?}",task_entry(56))

    
}

