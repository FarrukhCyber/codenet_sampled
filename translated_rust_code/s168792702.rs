use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut numbers = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        let a = numbers.next().unwrap();
        let b = numbers.next().unwrap();
        let c = numbers.next().unwrap();

        if a < b && b < c {
            println!("Yes");
        } else {
            println!("No");
        }
    }
