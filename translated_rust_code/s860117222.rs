use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();

        let d = c - a - b;
        if d > 0 && 4 * a * b < d * d {
            println!("Yes");
        } else {
            println!("No");
        }
    }
