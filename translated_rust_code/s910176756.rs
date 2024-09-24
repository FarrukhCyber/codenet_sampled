use std::io;

    fn main() {
        let mut input = String::new();

        loop {
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let mut iter = input.split_whitespace();

            let a: i32 = match iter.next().unwrap().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            let op = match iter.next() {
                Some(op) => op.chars().next().unwrap(),
                None => break,
            };

            let b: i32 = match iter.next().unwrap().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            let c = match op {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => break,
            };

            println!("{}", c);

            input.clear();
        }
    }
