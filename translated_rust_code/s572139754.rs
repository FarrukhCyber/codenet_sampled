use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i32 = input.trim().parse().expect("Please type a number!");

        if n % 1000 == 0 {
            println!("0");
        } else {
            println!("{}", (n / 1000 + 1) * 1000 - n);
        }
    }
