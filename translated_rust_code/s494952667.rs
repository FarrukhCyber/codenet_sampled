use std::io;

    fn main() {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let reversed_input: String = input.chars().rev().collect();

        println!("{}", reversed_input);
    }
