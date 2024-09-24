use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let s: i32 = input.trim().parse().expect("Please type a number!");

        let hours = s / 3600;
        let minutes = (s % 3600) / 60;
        let seconds = s % 60;

        println!("{}:{}:{}", hours, minutes, seconds);
    }
