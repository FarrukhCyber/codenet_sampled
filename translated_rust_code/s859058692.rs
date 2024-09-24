use std::io;

    fn main() {
        let mut sum = 0;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i32 = n.trim().parse().expect("Please type a number!");

        for _ in 0..n {
            let mut m = String::new();
            io::stdin().read_line(&mut m).expect("Failed to read line");
            let m: i32 = m.trim().parse().expect("Please type a number!");

            sum += m;
            min = min.min(m);
            max = max.max(m);
        }

        println!("{} {} {}", min, max, sum);
    }
    
    The provided Rust code reads a number `n` from the standard input, then reads `n` numbers from the standard input, calculates their sum, finds the minimum and maximum number, and prints them out. The code uses the `std::io` module for input/output operations and the `min` and `max` methods of the `i32` type to find the minimum and maximum number.
