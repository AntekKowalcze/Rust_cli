use crate::command_executions::{self};

pub fn matching_input(input_vec: Vec<&str>) {
    match input_vec.get(0) {
        Some(&"echo") => command_executions::echo(input_vec),
        Some(&"exit") => command_executions::exit(),
        Some(&"cd") => todo!(),
        Some(_) => panic!("Wrong input"),
        None => panic!("No input"),
    }
}
