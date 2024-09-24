use std::io;

    fn swap(a: &mut i32, b: &mut i32) {
        let tmp = *a;
        *a = *b;
        *b = tmp;
    }

    fn main() {
        let mut input = String::new();
        let mut a = 0;
        let mut b = 0;

        loop {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().expect("Please type numbers!")).collect();

            if numbers.len() != 2 {
                println!("Please enter two numbers");
                continue;
            }

            a = numbers[0];
            b = numbers[1];

            if a == 0 && b == 0 {
                break;
            }

            if b < a {
                swap(&mut a, &mut b);
            }

            println!("{} {}", a, b);
        }
    }
