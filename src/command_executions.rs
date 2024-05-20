use std::{
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

                    let mut marked_directory_list: Vec<PathBuf> = Vec::new();

                    match current_dir() {
                        Ok(current_dir) => {
                            comparing_files(
                                file_name_list,
                                file_name,
                                marked_directory_list,
                                current_dir,
                                starting_point_path,
                                directory_list,
                            );
                            // match file_name_list.iter().find(|found_element| {
                            //     found_element.file_name() == OsString::from(&file_name)
                            // }) {
                            //     Some(found_element) => {
                            //         println!(
                            //             "Found this file at {}",
                            //             found_element.path().display()
                            //         )
                            //     }
                            //     None => {
                            //         for directory in &directory_list {
                            //             if directory.marked == false {
                            //                 set_current_dir(&directory.path).unwrap_or_else(|_| {
                            //                     println!("Cant be error, path given in list");
                            //                     main()
                            //                 });
                            //             }
                            //         }
                            //         //     0. Nie ma pliku zrób liste
                            //         //TODO 1. Sprawdź czy są jakieś inne, nie zaznaczone foldery
                            //         //     2. Jeśli są wejdź do pierwszego, wyczyść listę, sprawdź pliki, zrób listę
                            //         //     3. Powtórz 1-2 dopóki nie ma folderów.
                            //         //     4. jeśli nie ma folderów i nie znaleziono pliku ,

                            //         //sprawdź czy są jakieś nie marked i czy jest znalezione
                            //         //wyczyść listę nazw plików
                            //         //przejdź do kolejnego nie oznaczonego folderu
                            //     }
                            // }

                            //set current dir to starting
                            //list files search for one set_current_dir(starting_point)
                            //if found print path
                            //else
                            //list directories
                            //choose one
                            //check if its marked
                            //if its marked choose next
                            //if all are marked mark this directory as searched, go to starting point
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
fn no_flag_expected(input_vec: &Vec<&str>, last_flag_index: usize) {
    //This function look if there is more content that it should be in input
    if let Some(_) = input_vec.get(last_flag_index + 1) {
        println!("Too mutch arguments");
        main()
    }
}

fn prototype(input_vec: &Vec<&str>) {
    if let Some(file_name) = input_vec.get(2) {
        let file_name = file_name.to_string();
        if let Some(starting_point) = input_vec.get(3) {
            let starting_point_path = Path::new(*starting_point);
            set_current_dir(starting_point_path).unwrap_or_else(|e| {
                println!("{}", e);
                main()
            });

            let mut file_name_list: Vec<DirEntry> = Vec::new();
            let mut directory_list: Vec<PathBuf> = Vec::new();
            let mut marked_directory_list: Vec<PathBuf> = Vec::new();

            match current_dir() {
                Ok(mut current_dir) => {
                    if let Ok(iterator) = read_dir(&current_dir) {
                        for item in iterator {
                            if let Ok(item) = item {
                                if item.path().is_file() {
                                    file_name_list.push(item);
                                } else if item.path().is_dir() {
                                    directory_list.push(item.path());
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
                                println!("Found this file at {}", found_element.path().display())
                            }
                            None => {
                                for directory in &directory_list {
                                    if !marked_directory_list.contains(&directory) {
                                        set_current_dir(directory).expect("hardcoded directory should exist/eventually permission problem");
                                        //checking files and listing dirs (create new vectors)
                                    }
                                }
                                //wyjdź poziom do góry i sprawdź foldery
                                if current_dir == starting_point_path {
                                    println!("Nie znaleziono pliku");
                                } else {
                                    marked_directory_list.clear();
                                    marked_directory_list.push(current_dir.clone());
                                    current_dir.pop();
                                    set_current_dir(current_dir).unwrap_or_else(|e| {
                                        println!("Cant get into directory, {}", e);
                                        main()
                                    });

                                    let (directory_list_new, option_file_list) =
                                        listing_directories_and_or_files(false);
                                    if let Some(file_list) = option_file_list {
                                        println!("smth went wrong when calling function")
                                    } else {
                                        //recursive_call_push_dir_list
                                    }
                                    //recursive call
                                    //
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
    mut marked_directory_list: Vec<PathBuf>,
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
                    println!("Found this file at {}", found_element.path().display())
                }
                None => {
                    if current_dir == starting_point_path {
                        println!(
                            "File not found in this tree {}",
                            starting_point_path.display()
                        )
                    } else {
                        reversing_graph(marked_directory_list, directory_list, current_dir);
                    }

                    // for directory in &directory_list {
                    //     if !marked_directory_list.contains(&directory) {
                    //         set_current_dir(directory).expect(
                    //             "hardcoded directory should exist/eventually permission problem",
                    //         );
                    //         //checking files and listing dirs (create new vectors)
                    //     }
                    // }
                }
            }
        }
        None => {
            panic!("there should be file list");
        }
    }
}
fn reversing_graph(
    mut marked_directory_list: Vec<PathBuf>,
    directory_list: Vec<PathBuf>,
    current_dir: PathBuf,
) {
    //1. dodaj siebie do sprawdzonych
    marked_directory_list.push(current_dir);

    //2. stwórz wektor directory których nie ma w directory list,

    for directory in directory_list {
        if !marked_directory_set.contains(&directory) {
            unchecked_directory_list.push(directory);
        }
    }
}
//Jeśli jest pusty, idź w góre wywołaj tworzenie wektora bez plików opróżnij marked_directory_list idź do 1.
//Jeśli nie
//3. wejdź do directory.0
//4. ustaw nowe directory jako nazwę current_directory
//5. Stwórz nowy wektor plików i directory (wywołaj funkcję)
//6. wywołaj sprawdzanie plików
