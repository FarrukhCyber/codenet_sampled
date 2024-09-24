use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().trim().parse().expect("Please type a number!");
        let b: i32 = iter.next().unwrap().trim().parse().expect("Please type a number!");
        let c: i32 = iter.next().unwrap().trim().parse().expect("Please type a number!");

        if (a < c && c < b) || (a > c && c > b) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
