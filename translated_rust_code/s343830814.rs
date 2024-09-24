use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let len = input.len();
        println!("{}{}{}", input.chars().next().unwrap(), len - 3, input.chars().nth(len - 2).unwrap());
    }
