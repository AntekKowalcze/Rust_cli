//! to do
//! moving in computer paths ect.
//! showing current directory and user
//!

//! }echo: repeats input
//! cat: concatenates files
//! ls: lists directories
//! find: locates files or directories
//! grep: matches text in files
//! cd: going to other directory

use main_menu::printing_information::printing_information;
use main_menu::user_input::creating_input;
mod main_menu;
mod pattern_matching;
mod test;

fn main() {
    printing_information();
    let user_input: String = creating_input().expect("hardcoded can't be error");
    let input_vec: Vec<&str> = user_input.split_whitespace().collect();
    pattern_matching::pattern_matching(input_vec);
    println!();
}
