use std::io;

    fn main() {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let x: i32 = input.trim().parse().expect("Please type a number!");

        let kotae = x * x * x;

        println!("{}", kotae);
    }
