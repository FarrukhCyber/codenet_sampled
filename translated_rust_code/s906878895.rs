use std::io;

    fn main() {
        let mut input = String::new();

        while io::stdin().read_line(&mut input).unwrap() != 0 {
            let numbers: Vec<i32> = input.trim().split_whitespace()
                                         .map(|s| s.parse().unwrap())
                                         .collect();

            let a = numbers[0];
            let b = numbers[1];

            let x = a + b;
            let mut w = 0;

            let mut temp = x;
            while temp != 0 {
                w += 1;
                temp /= 10;
            }

            println!("{}", w);

            input.clear();
        }
    }
