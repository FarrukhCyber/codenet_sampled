use std::io;

    fn main() {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();

        let h: i32 = iter.next().unwrap().trim().parse().expect("Please type a number!");
        let r: i32 = iter.next().unwrap().trim().parse().expect("Please type a number!");

        if r > -h {
            println!("1");
        } else if r == -h {
            println!("0");
        } else {
            println!("-1");
        }
    }
