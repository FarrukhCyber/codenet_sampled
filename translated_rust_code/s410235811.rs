use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i32 = input.trim().parse().expect("Please type a number!");

        let result = if n % 2 == 0 {
            n / 2
        } else {
            (n / 2) + 1
        };

        println!("{}", result);
    }
