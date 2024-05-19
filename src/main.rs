//! TODO
//! find: locates files or directories
//! grep: matches text in files
use main_menu::printing_information::printing_information;
use main_menu::user_input::creating_input;

mod command_executions;
mod main_menu;
mod matching_input;
mod test;

pub fn main() {
    //TODO pierwsza wyśiwetlana ściezka, aktualna ściezka
    loop {
        printing_information();
        let user_input: String = creating_input().expect("hardcoded can't be error");
        let input_vec: Vec<&str> = user_input.split_whitespace().collect();
        matching_input::matching_input(input_vec);
        println!();
    }
}
