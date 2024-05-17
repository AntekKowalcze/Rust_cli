//! to do
//! moving in computer paths ect.
//! }echo: repeats input
//! cat: concatenates files
//! ls: lists directories
//! find: locates files or directories
//! grep: matches text in files
//! cd: going to other directory
use main_menu::printing_information::printing_information;
use main_menu::user_input::creating_input;

mod command_executions;
mod main_menu;
mod matching_input;
mod test;

fn main() {
    loop {
        printing_information();
        let user_input: String = creating_input().expect("hardcoded can't be error");
        let input_vec: Vec<&str> = user_input.split_whitespace().collect();
        matching_input::matching_input(input_vec);
        println!();
    }
}
