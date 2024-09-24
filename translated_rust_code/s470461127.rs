use std::io;

    fn main() {
        let mut x = String::new();
        let mut y = String::new();

        io::stdin().read_line(&mut x).expect("Failed to read line");
        io::stdin().read_line(&mut y).expect("Failed to read line");

        let x: i32 = x.trim().parse().expect("Please type a number!");
        let y: i32 = y.trim().parse().expect("Please type a number!");

        println!("{} {}", x * y, (2 * x) + (2 * y));
    }
