use std::{
    fs::File,
    io::{self, Write},
};
pub fn echo(input_vec: Vec<&str>) {
    for iterator in input_vec[1..].iter() {
        if *iterator == ">>" {
            let mut input_iterator = input_vec.iter(); //tworzenie iteratora na wektorze

            let redirection = match input_iterator.position(|x| *x == ">>") {
                //znalezienie >> znaku i jego pozycji
                Some(value) => value,
                None => panic!("there is no redirection while it's called"),
            };

            let string_we_want_to_write: String = input_vec[1..redirection].join(" ");
            //znajdowanie znaku przekierowania w wektorze
            let file_name = input_vec[redirection + 1];

            match File::open(file_name) {
                Ok(_) => File::open(file_name)
                    .expect("hardcoded no errors in opening file")
                    .write(string_we_want_to_write.as_bytes())
                    .expect("failed to write into a file"),
                Err(_) => File::create(file_name)
                    .expect("cannont open a file")
                    .write(string_we_want_to_write.as_bytes())
                    .expect("failed to write to file"),
            };
        } else {
            print!("{} ", iterator);
            io::stdout().flush().expect("failed to flush");
        }
    }
}
pub fn exit() {
    std::process::exit(0)
}
