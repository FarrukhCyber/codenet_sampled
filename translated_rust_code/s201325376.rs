use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().trim().parse().unwrap();
        let b: i32 = iter.next().unwrap().trim().parse().unwrap();
        let c: i32 = iter.next().unwrap().trim().parse().unwrap();

        let mut count = 0;
        for i in a..=b {
            if c % i == 0 {
                count += 1;
            }
        }

        println!("{}", count);
    }
