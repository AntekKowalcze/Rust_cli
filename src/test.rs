#[cfg(test)]
mod test {

    use crate::{
        main_menu::{printing_information, user_input::creating_input},
        matching_input,
    };
    #[test]
    fn input_test() {
        printing_information::printing_information();
        let user_input: String = creating_input().expect("hardcoded cant be error");
        let input_vec: Vec<&str> = user_input.split_whitespace().collect();

        assert_eq!(input_vec, vec!["Some", "text"])
    }
    #[test]
    #[should_panic]
    fn patter_check() {
        let input_vec = vec!["some"];
        matching_input::matching_input(input_vec);
    }
}
