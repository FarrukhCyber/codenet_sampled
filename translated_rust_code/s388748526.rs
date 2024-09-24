use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let r: f64 = input.trim().parse().expect("Please type a number!");
        println!("{} {}", r * r * std::f64::consts::PI, 2.0 * std::f64::consts::PI * r);
    }
