use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let r: i32 = iter.next().unwrap().parse().unwrap();

        if n >= 10 {
            println!("{}", r);
        } else {
            println!("{}", r + 100 * (10 - n));
        }
    }
