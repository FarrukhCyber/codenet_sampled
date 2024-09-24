use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        for i in 0..input.len() - 1 {
            if &input[i..i + 2] == "AC" {
                println!("Yes");
                return;
            }
        }

        println!("No");
    }
