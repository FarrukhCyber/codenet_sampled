use std::io;

    fn main() {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        let first_char = input.chars().next().unwrap();
        let last_char = input.chars().last().unwrap();
        let length = input.len() - 2;

        println!("{}{}{}", first_char, length, last_char);
    }
