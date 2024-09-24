use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().trim().parse().expect("Please type a number!");
        let a: i32 = iter.next().unwrap().trim().parse().expect("Please type a number!");
        println!("{}", if x >= a { 10 } else { 0 });
    }
