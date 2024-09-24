use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();

        if a < b && b < c {
            println!("Yes");
        } else {
            println!("No");
        }
    }
