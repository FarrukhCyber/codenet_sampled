use std::io;

    fn main() {
        let mut sum = 0;
        let mut input = String::new();

        loop {
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let trimmed = input.trim();

            if trimmed == "0" {
                break;
            }

            for c in trimmed.chars() {
                sum += c.to_digit(10).unwrap() as i32;
            }

            println!("{}", sum);
            sum = 0;
            input.clear();
        }
    }
