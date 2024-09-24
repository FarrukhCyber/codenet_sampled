use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let numbers: Vec<i32> = input.split_whitespace()
                                     .map(|s| s.trim().parse().expect("Please type a number!"))
                                     .collect();

        let m1 = numbers[0];
        let d1 = numbers[1];
        let m2 = numbers[2];
        let d2 = numbers[3];

        if m1 != m2 {
            println!("1");
        } else {
            println!("0");
        }
    }
