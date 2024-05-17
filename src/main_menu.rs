pub mod user_input {
    use std::io;

    pub fn creating_input() -> Result<String, io::Error> {
        let mut user_input: String = String::new();
        let read_line_success: Result<String, std::io::Error> = {
            match io::stdin().read_line(&mut user_input) {
                Ok(_) => Ok(user_input.trim().to_owned()),
                Err(err) => {
                    println!("No command found, {}", err);
                    creating_input()
                }
            }
        };
        read_line_success
    }
}
pub mod printing_information {
    use std::{
        env,
        io::{self, Write},
    };
    use whoami::username;
    pub fn printing_information() {
        let path = env::current_dir().expect("dont know how to handle it rn");
        let username = username();
        print!("{} u{} ", path.display(), username);
        io::stdout().flush().expect("failed to flush"); // flushowanie print aby wyświetliło się odrazu przed inuptem
    }
}

// #[cfg(test)]
// mod test {

//     use super::user_input::creating_input;

//     fn giving_input() -> MyError {
//         match creating_input() {
//             Ok(value) => MyError::OkValue(value),
//             Err(_) => MyError::ErrValue(true),
//         }
//     }
//     #[test]
//     fn user_input_test() {
//         assert_eq!(giving_input(), MyError::OkValue(String::from("Some shit")));
//     }

//     #[derive(PartialEq, Debug)]
//     enum MyError {
//         OkValue(String),
//         ErrValue(bool),
//     }
// }
