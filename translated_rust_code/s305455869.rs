use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().parse().expect("Please type a number!");
        let b: i32 = iter.next().unwrap().parse().expect("Please type a number!");

        let mut ans = 0;
        if a >= 13 {
            ans = b;
        } else if a >= 6 {
            ans = b / 2;
        }

        println!("{}", ans);
    }
