use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut chars: Vec<char> = input.chars().collect();

        chars[5] = ' ';
        chars[13] = ' ';

        let output: String = chars.into_iter().collect();

        println!("{}", output);
    }
