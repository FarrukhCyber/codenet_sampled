use std::io;

    fn main() {
        let p = 3.141592653589;

        let mut r = String::new();
        io::stdin().read_line(&mut r).expect("Failed to read line");
        let r: f64 = r.trim().parse().expect("Please type a number!");

        let s = r * r * p;
        let l = 2.0 * p * r;

        println!("{} {}", s, l);
    }
