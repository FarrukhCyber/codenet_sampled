use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().expect("Please type a number!")).collect();

        let n = numbers[0];
        let m = numbers[1];

        println!("{}", (n - 1) * (m - 1));
    }
