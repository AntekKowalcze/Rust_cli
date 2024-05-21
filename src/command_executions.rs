use std::{
    collections::HashSet,
    env::{current_dir, set_current_dir},
    ffi::OsString,
    fs::{self, read_dir, DirEntry, File},
    io::{self, Write},
    path::{Path, PathBuf},
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

                    let (mut directory_list, mut file_name_list) =
                        listing_directories_and_or_files(true);

                    let marked_directory_set: std::collections::HashSet<PathBuf> = HashSet::new();

                    match current_dir() {
                        Ok(current_dir) => {
                            comparing_files(
                                file_name_list,
                                file_name,
                                marked_directory_set,
                                current_dir,
                                starting_point_path,
                                directory_list,
                            );
                        }
                        Err(e) => {
                            println!("{}", e);
                            main()
                        }
                    }
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

fn listing_directories_and_or_files(files_needed: bool) -> (Vec<PathBuf>, Option<Vec<DirEntry>>) {
    let mut directory_list: Vec<PathBuf> = Vec::new();
    let mut file_list: Vec<std::fs::DirEntry> = Vec::new();
    if let Ok(current_dir) = current_dir() {
        if let Ok(iterator) = read_dir(current_dir) {
            for item in iterator {
                if let Ok(item) = item {
                    if files_needed {
                        if item.path().is_file() {
                            file_list.push(item);
                        } else if item.path().is_dir() {
                            directory_list.push(item.path());
                        }
                    } else if item.path().is_dir() {
                        directory_list.push(item.path());
                    }
                }
            }
            if files_needed {
                (directory_list, Some(file_list))
            } else {
                (directory_list, None)
            }
        } else {
            println!("couldnt read file");
            (directory_list, None)
        }
    } else {
        panic!("Couldnt get current path")
    }
}
fn comparing_files(
    file_name_list: Option<Vec<DirEntry>>,
    file_name: String,
    mut marked_directory_set: HashSet<PathBuf>,
    current_dir: PathBuf,
    starting_point_path: &Path,
    directory_list: Vec<PathBuf>,
) {
    match file_name_list {
        Some(file_name_list) => {
            match file_name_list
                .iter()
                .find(|found_element| *found_element.file_name() == OsString::from(&file_name))
            {
                Some(found_element) => {
                    println!("Found this file at {}", found_element.path().display());
                    marked_directory_set.insert(current_dir.clone());
                    main();
                }
                None => {
                    if marked_directory_set.contains(&current_dir)
                        && current_dir == starting_point_path
                    {
                        println!(
                            "File not found in this tree {}",
                            starting_point_path.display()
                        )
                    } else {
                        marked_directory_set.insert(current_dir.clone());
                        reversing_graph(
                            marked_directory_set,
                            directory_list,
                            current_dir,
                            file_name,
                            starting_point_path,
                        );
                    }
                }
            }
        }
        None => {
            panic!("there should be file list");
        }
    }
}
fn reversing_graph(
    mut marked_directory_set: HashSet<PathBuf>,
    directory_list: Vec<PathBuf>,
    mut current_dir: PathBuf,
    file_name: String,
    starting_point_path: &Path,
) {
    //PROGRAM JAKBY BLOKUJE SIĘ NA JEDNEJ ŚCIEŻCE
    //1. dodaj siebie do sprawdzonych
    marked_directory_set.insert(current_dir.clone());

    //2. stwórz wektor directory których nie ma w directory list,

    let mut unchecked_directory_list: Vec<PathBuf> = Vec::new();
    for directory in directory_list {
        if !marked_directory_set.contains(&directory) {
            unchecked_directory_list.push(directory);
        }
    } //WORKS WELL

    //Jeśli jest pusty, idź w góre wywołaj tworzenie wektora bez plików opróżnij marked_directory_list idź do 1.
    if unchecked_directory_list.is_empty() {
        if current_dir == starting_point_path {
            // Jeśli wróciliśmy do punktu startowego i lista nieodwiedzonych katalogów jest pusta, przerywamy rekurencję
            println!(
                "File not found in this tree {}",
                starting_point_path.display()
            );
            return;
        }

        current_dir.pop();

        let new_dir = current_dir;

        set_current_dir(new_dir.clone()).unwrap_or_else(|e| {
            println!("Couldnt get into higher directory {e} ");
            main()
        });

        let (mut directory_list, mut file_name_list) = listing_directories_and_or_files(false);
        reversing_graph(
            marked_directory_set,
            directory_list,
            new_dir,
            file_name,
            starting_point_path,
        );
    }
    //Jeśli nie
    else {
        //3. wejdź do directory.0
        current_dir = unchecked_directory_list
            .get(0)
            .expect("CANT BE EMPTY")
            .to_owned();

        set_current_dir(current_dir.clone()).unwrap_or_else(|e| {
            println!("Couldnt get into lower directory {e} ");
            main()
        });

        //4. ustaw nowe directory jako nazwę current_directory
        let new_dir = current_dir;
        //5. Stwórz nowy wektor plików i directory (wywołaj funkcję)
        let (mut directory_list, mut file_name_list) = listing_directories_and_or_files(true);
        comparing_files(
            file_name_list,
            file_name,
            marked_directory_set,
            new_dir,
            starting_point_path,
            directory_list,
        );
    }
}
fn no_flag_expected(input_vec: &Vec<&str>, last_flag_index: usize) {
    //This function look if there is more content that it should be in input
    if let Some(_) = input_vec.get(last_flag_index + 1) {
        println!("Too mutch arguments");
        main()
    }
}
