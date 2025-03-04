use crate::data::file_management::*;
use crate::interface::task_inteface::RegisterTask;
use std::path::Path;

pub fn data_management(path: &str) -> Vec<RegisterTask> {
    let path_: &Path = Path::new(path);
    let (content_file,status_file): (String,bool) = read_file(path_); 
    let content_task: String = read_file_with_status(content_file, status_file);
    let list_content_task: Vec<Vec<String>> = convert_to_do_lists(content_task);
    let task_log: Vec<RegisterTask> = task_log(list_content_task);
    
    //println!("{:?}",task_log);
    task_log
}