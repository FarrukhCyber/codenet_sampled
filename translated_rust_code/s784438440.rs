use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().trim().parse().unwrap();
        let b: i32 = iter.next().unwrap().trim().parse().unwrap();
        println!("{}", ((b-1)+(a-2))/(a-1));
    }
