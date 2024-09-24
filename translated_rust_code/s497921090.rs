use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: u64 = input.trim().parse().unwrap();
        let b: u64 = (n * (n - 1)) / 2;
        println!("{}", b);
    }
