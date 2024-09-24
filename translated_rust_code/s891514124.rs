use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let s: i32 = input.trim().parse().expect("Please type a number!");

        if s < 60 {
            println!("0:0:{}", s);
        } else if s < 3600 {
            println!("0:{}:{}", s / 60, s % 60);
        } else {
            println!("{}:{}:{}", s / 3600, (s % 3600) / 60, (s % 60) % 60);
        }
    }
