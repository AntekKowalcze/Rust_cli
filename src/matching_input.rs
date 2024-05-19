use crate::command_executions::{self};

pub fn matching_input(input_vec: Vec<&str>) {
    match input_vec.get(0) {
        Some(&"echo") => command_executions::echo(input_vec),
        Some(&"exit") => command_executions::exit(),
        Some(&"goto") => command_executions::changing_directory(input_vec),
        Some(&"list") => command_executions::list_directories(input_vec),
        Some(&"display") => command_executions::concatanate_file(input_vec),
        Some(&"find") => command_executions::find_file_or_content_in_file(input_vec),
        Some(_) => println!("Command not found"),
        None => println!("Command not found"),
    }
}
