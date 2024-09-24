use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<_> = input.split_whitespace()
                                   .map(|s| s.parse::<i32>().expect("Please type a number!"))
                                   .collect();

        let sheep = numbers[0];
        let wolf = numbers[1];

        if wolf >= sheep {
            println!("unsafe");
        } else {
            println!("safe");
        }
    }
