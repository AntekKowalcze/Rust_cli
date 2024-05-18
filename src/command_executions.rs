use std::{
    env::{current_dir, set_current_dir},
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

use crate::main;

pub fn echo(input_vec: Vec<&str>) {
    for iterator in input_vec[1..].iter() {
        if *iterator == ">>" {
            let mut input_iterator = input_vec.iter(); //making iterator over vec

            let redirection = match input_iterator.position(|x| *x == ">>") {
                //founding >> sign and its position
                Some(value) => value,
                None => panic!("there is no redirection while it's called"),
            };

            let string_we_want_to_write: String = input_vec[1..redirection].join(" ");
            //getting text we want to write
            let file_name = input_vec[redirection + 1];

            let mut file = File::options()
                .create(true) //when file doesnt exist it creates new
                .read(true)
                .write(true)
                .append(true) //text will be appended to already existing, if you want to overwrite use truncate(true)
                .open(file_name)
                .expect("Failed to open a file");

            file.write(string_we_want_to_write.as_bytes())
                .expect("failed to write into a file");
        } else {
            print!("{} ", iterator);
            io::stdout().flush().expect("failed to flush");
        }
    }
}

pub fn exit() {
    std::process::exit(0) //exiting actual process
}

pub fn changing_directory(input_vec: Vec<&str>) {
    let last_flag_index: usize = 1;
    no_flag_expected(&input_vec, last_flag_index);
    if let Some(go_to_path_str) = input_vec.get(1) {
        if *go_to_path_str == ".." {
            if let Ok(mut path) = current_dir() {
                path.pop();

                set_current_dir(path).unwrap_or_else(|e| println!("{}", e))
            } else {
                println!("Error there is no current path");
            }
        } else {
            let path = Path::new(*go_to_path_str);
            set_current_dir(path).unwrap_or_else(|e| println!("{}", e));
        }
    } else {
        println!("No directory specified");
        main()
    }
}
pub fn list_directories(input_vec: Vec<&str>) {
    let last_flag_index: usize = 1;
    no_flag_expected(&input_vec, last_flag_index);
    let files_and_directories = fs::read_dir(".").unwrap();
    if let Some(flag) = input_vec.get(1) {
        match *flag {
            "-d" => {
                for object in files_and_directories {
                    match object {
                        Ok(object) => {
                            if object.path().is_dir() {
                                println!("{}", object.path().display());
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                            main()
                        }
                    }
                }
            }
            "-f" => {
                for object in files_and_directories {
                    match object {
                        Ok(object) => {
                            if object.path().is_file() {
                                println!("{}", object.path().display());
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                            main()
                        }
                    }
                }
            }
            _ => {
                println!("Error - Wrong flag!");
                main()
            }
        }
    } else {
        for object in files_and_directories {
            match object {
                Ok(object) => {
                    println!("{}", object.path().display());
                }

                Err(e) => {
                    println!("{}", e);
                    main()
                }
            }
        }
    }
}
fn no_flag_expected(input_vec: &Vec<&str>, last_flag_index: usize) {
    //This function look if there is more content that it should be in input
    if let Some(_) = input_vec.get(last_flag_index + 1) {
        println!("Wrong command");
        main()
    }
}
