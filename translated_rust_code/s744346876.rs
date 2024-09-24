use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().expect("Please type a number!")).collect();

        if numbers.len() == 2 && numbers[0] < 10 && numbers[1] < 10 {
            println!("{}", numbers[0] * numbers[1]);
        } else {
            println!("-1");
        }
    }
