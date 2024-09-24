use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().expect("Please type a number!")).collect();

        let a = numbers[0];
        let b = numbers[1];
        let c = numbers[2];
        let d = numbers[3];

        let i = a * b;
        let j = c * d;

        if i > j {
            println!("{}", i);
        } else {
            println!("{}", j);
        }
    }
