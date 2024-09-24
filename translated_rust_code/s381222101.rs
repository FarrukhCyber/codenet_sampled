use std::io;

    fn main() {
        let mut input = String::new();
        println!("Enter three integers separated by spaces:");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let h = iter.next().unwrap();
        println!("{}", (a + b) * h / 2);
    }
