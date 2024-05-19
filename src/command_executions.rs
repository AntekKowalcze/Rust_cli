use std::{
    env::{current_dir, set_current_dir},
    ffi::OsString,
    fs::{self, read_dir, DirEntry, File},
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

pub fn concatanate_file(input_vec: Vec<&str>) {
    let last_flag_index = 1;
    no_flag_expected(&input_vec, last_flag_index);
    if let Some(path_to_a_file) = input_vec.get(1) {
        let path_to_a_file = Path::new(*path_to_a_file);
        if let Ok(file_content) = fs::read_to_string(path_to_a_file) {
            println!("{}", file_content);
        } else {
            println!("File not found");
            main()
        }
    } else {
        println!("To less arguments");
        main()
    }
}

pub fn find_file_or_content_in_file(input_vec: Vec<&str>) {
    //TODO dodać że bez drugiego argumentu bieże domyślny akturalną ścieżkę
    match input_vec.get(1) {
        Some(&"-f") => {
            if let Some(file_name) = input_vec.get(2) {
                let file_name = file_name.to_string();
                if let Some(starting_point) = input_vec.get(3) {
                    let starting_point_path = Path::new(*starting_point);
                    set_current_dir(starting_point_path).unwrap_or_else(|e| {
                        println!("{}", e);
                        main()
                    });

                    let mut file_name_list: Vec<DirEntry> = Vec::new();
                    let mut directory_list: Vec<Directory> = Vec::new();

                    match current_dir() {
                        Ok(current_dir) => {
                            if let Ok(iterator) = read_dir(&current_dir) {
                                for item in iterator {
                                    if let Ok(item) = item {
                                        if item.path().is_file() {
                                            file_name_list.push(item);
                                        } else if item.path().is_dir() {
                                            let directory_path = Directory {
                                                path: item.path(),
                                                marked: false,
                                            };
                                            directory_list.push(directory_path);
                                        }
                                    } else {
                                        println!("Can't get entry from iterator");
                                        main()
                                    }
                                }
                                match file_name_list.iter().find(|found_element| {
                                    *found_element.file_name() == OsString::from(&file_name)
                                }) {
                                    Some(found_element) => {
                                        println!(
                                            "Found this file at {}",
                                            found_element.path().display()
                                        )
                                    }
                                    None => {
                                        for directory in &directory_list {
                                            if directory.marked == false {
                                                set_current_dir(&directory.path).unwrap_or_else(
                                                    |e| {
                                                        println!(
                                                            "Cant be error, path given in list"
                                                        );
                                                        main()
                                                    },
                                                );
                                            }
                                        }
                                        //     0. Nie ma pliku zrób liste
                                        //TODO 1. Sprawdź czy są jakieś inne, nie zaznaczone foldery
                                        //     2. Jeśli są wejdź do pierwszego, wyczyść listę, sprawdź pliki, zrób listę
                                        //     3. Powtórz 1-2 dopóki nie ma folderów.
                                        //     4. jeśli nie ma folderów i nie znaleziono pliku ,

                                        //sprawdź czy są jakieś nie marked i czy jest znalezione
                                        //wyczyść listę nazw plików
                                        //przejdź do kolejnego nie oznaczonego folderu
                                    }
                                }
                            } else {
                                println!("Unable to read directory");
                                main()
                            }
                        }
                        Err(e) => {
                            println!("{}", e);
                            main()
                        }
                    }

                    //set current dir to starting
                    //list files search for one set_current_dir(starting_point)
                    //if found print path
                    //else
                    //list directories
                    //choose one
                    //check if its marked
                    //if its marked choose next
                    //if all are marked mark this directory as searched, go to starting point
                } else {
                    println!("Starting point not specified");
                    main()
                }
            } else {
                println!("No file specified");
            }
        }
        Some(&"-c") => {}

        None | _ => {
            println!("Too less arguments");
            main()
        }
    }
}
fn checking_if_dir_contains_file(
    mut directory_list: Vec<Directory>,
    mut file_name_list: Vec<DirEntry>,
) -> (Option<Vec<Directory>>, Option<Vec<DirEntry>>) {
    match current_dir() {
        Ok(current_dir) => {
            if let Ok(iterator) = read_dir(&current_dir) {
                for item in iterator {
                    if let Ok(item) = item {
                        if item.path().is_file() {
                            file_name_list.push(item);
                        } else if item.path().is_dir() {
                            let directory_path = Directory {
                                path: item.path(),
                                marked: false,
                            };
                            directory_list.push(directory_path);
                        }
                    } else {
                        println!("Can't get entry from iterator");
                        main();
                        return (None, None);
                    }
                }
                (Some(directory_list), Some(file_name_list))
            } else {
                println!("Unable to read directory");
                main();
                (None, None)
            }
        }
        Err(e) => {
            println!("{}", e);
            main();
            (None, None)
        }
    }
}

fn no_flag_expected(input_vec: &Vec<&str>, last_flag_index: usize) {
    //This function look if there is more content that it should be in input
    if let Some(_) = input_vec.get(last_flag_index + 1) {
        println!("Too mutch arguments");
        main()
    }
}

struct Directory {
    path: std::path::PathBuf,
    marked: bool,
}
impl Directory {
    fn mark_true(&mut self) {
        self.marked = true;
    }
    fn is_marked(&self) -> bool {
        if self.marked == true {
            true
        } else {
            false
        }
    }
}
