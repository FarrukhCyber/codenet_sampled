use std::io;

    fn main() {
        let mut a = String::new();
        let mut b = String::new();

        println!("Enter two numbers:");
        io::stdin().read_line(&mut a).expect("Failed to read line");
        io::stdin().read_line(&mut b).expect("Failed to read line");

        let mut a: i32 = a.trim().parse().expect("Please type a number!");
        let mut b: i32 = b.trim().parse().expect("Please type a number!");

        if a == 1 {
            a = 14;
        }
        if b == 1 {
            b = 14;
        }

        if a < b {
            println!("Bob");
        } else if a > b {
            println!("Alice");
        } else {
            println!("Draw");
        }
    }
