use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: i32 = input.trim().parse().expect("Please type a number!");

        if x == 0 {
            println!("1");
        }
        if x == 1 {
            println!("0");
        }
    }
