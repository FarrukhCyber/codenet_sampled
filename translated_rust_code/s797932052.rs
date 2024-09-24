use std::io;

    fn main() {
        let mut input = String::new();
        let mut a = String::new();
        let mut b = String::new();

        println!("Enter first number:");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        let a: i32 = a.trim().parse().expect("Please type a number!");

        println!("Enter second number:");
        io::stdin().read_line(&mut b).expect("Failed to read line");
        let b: i32 = b.trim().parse().expect("Please type a number!");

        if (a * b) % 2 == 0 {
            println!("Even");
        } else {
            println!("Odd");
        }
    }
