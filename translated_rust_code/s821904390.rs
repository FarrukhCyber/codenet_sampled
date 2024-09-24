use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut in_seconds: i32 = input.trim().parse().expect("Please type a number!");

        let hours = in_seconds / 3600;
        in_seconds %= 3600;

        let minutes = in_seconds / 60;
        in_seconds %= 60;

        let seconds = in_seconds;

        println!("{}:{}:{}", hours, minutes, seconds);
    }
