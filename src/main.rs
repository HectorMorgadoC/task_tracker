mod interface;
mod data;

use interface::task_inteface::RegisterTask;
use crate::data::data_management::data_management;
use std::io::stdin;

fn main() {
    
    let data_task: Vec<RegisterTask> = data_management("src/data.json");

    let mut menu: String = String::new();
    println!("TASK LOGS");1
    println!("Enter 1 if you want to see the list of tasks");
    println!("Enter 2 if you want to enter task");
    stdin().read_line(&mut menu).unwrap_or(0);
    

    println!("{}",menu);
}

