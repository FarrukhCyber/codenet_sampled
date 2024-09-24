use std::io;

    fn main() {
        let mut i = 1;
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            if input == 0 {
                break;
            }
            println!("Case {}: {}", i, input);
            i += 1;
        }
    }
