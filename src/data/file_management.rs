use std::{fs::File, io::Read};
use crate::interface::task_inteface::RegisterTask;
use std::path::Path;


pub  fn read_file(path: &Path) -> (String,bool) {
    let data_task = File::open(path);
    let mut task_list: String = String::new();
    match data_task {
        Ok(mut value) => {
            value.read_to_string(&mut task_list).expect("Error loading file");
            (task_list,true)
        }
        Err(error) => 
        (error.to_string(),false)
    }
}


pub fn read_file_with_status(content: String, success: bool ) -> String {
    if success {
        content
    } else {
        File::create("src/datas.json").expect("Error create file");
        format!("")
    }
}

pub fn convert_to_do_lists(file_reading: String) -> Vec<Vec<String>> {
    let mut list_of_processed_words: Vec<Vec<String>> = vec![];

    if file_reading.contains("]") && file_reading.contains("[") {
        let replace_bracket: String = file_reading.replace("[", "");

        let replace_bracket: String = replace_bracket.replace("]", "");

        let separate_by_key: Vec<&str> = replace_bracket.split("},").collect();

        let remove_open_key: Vec<String> = separate_by_key
            .iter()
            .map(|&e| e.replace("{", ""))
            .collect();

        let remove_close_key: Vec<String> =
            remove_open_key.iter().map(|e| e.replace("}", "")).collect();

        let remove_line_break: Vec<String> = remove_close_key
            .iter()
            .map(|e| e.replace("\n", ""))
            .collect();

        let remove_space: Vec<String> = remove_line_break
            .iter()
            .map(|e| e.replace(" ", ""))
            .collect();

        let comma_for_two_dashes: Vec<String> =
            remove_space.iter().map(|e| e.replace(",", "--")).collect();

        for value in comma_for_two_dashes {
            let word_value_processed = value.clone();

            let data_by_double_hyphen: Vec<String> = word_value_processed
                .trim()
                .split("--")
                .map(|e| e.to_owned())
                .collect();

            list_of_processed_words.push(data_by_double_hyphen);
        }

        list_of_processed_words
    } else {
        list_of_processed_words
    }
}

pub fn task_log(to_do_list_text: Vec<Vec<String>>) -> Vec<RegisterTask> {
    let mut task: RegisterTask = RegisterTask::default();

    let mut task_list: Vec<RegisterTask> = vec![];

    if to_do_list_text.len() != 0 {
        for value in to_do_list_text {
            for value_second in value {
                let word_separated_by_colon: Vec<String> = value_second
                    .split(":")
                    .map(|e| e.to_owned().replace("\"", ""))
                    .collect();
    
                match word_separated_by_colon[0].as_str() {
                    "id" => task.id = word_separated_by_colon[1].parse().unwrap(),
                    "description" => task.description = word_separated_by_colon[1].clone(),
                    "status" => task.status_progress = word_separated_by_colon[1].clone(),
                    "createdAt" => task.created_at = word_separated_by_colon[1].clone(),
                    "updatedAt" => task.updated_at = word_separated_by_colon[1].clone(),
                    _ => {
                        println!("Error converting data to structure")
                    }
                }
            }
    
            task_list.push(task.clone());
        }
        task_list
    } else {
        task_list
    }
}

