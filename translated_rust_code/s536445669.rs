use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: i32 = iter.next().unwrap().trim().parse().unwrap();
        let k: i32 = iter.next().unwrap().trim().parse().unwrap();

        if n % k == 0 {
            println!("0");
        } else {
            println!("1");
        }
    }
