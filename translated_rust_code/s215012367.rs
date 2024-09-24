use std::io;

    fn main() {
        let mut a = String::new();
        let mut b = String::new();

        println!("Enter two numbers:");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        io::stdin().read_line(&mut b).expect("Failed to read line");

        let a: i32 = a.trim().parse().expect("Please type a number!");
        let b: i32 = b.trim().parse().expect("Please type a number!");

        let s = (a * b) - (a + b) + 1;

        println!("{}", s);
    }
