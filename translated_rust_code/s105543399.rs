use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i64 = n.trim().parse().expect("Please type a number!");

        let mut m = i64::MAX;
        let mut M = i64::MIN;
        let mut c = 0;

        for _ in 0..n {
            let mut a = String::new();
            io::stdin().read_line(&mut a).expect("Failed to read line");
            let a: i64 = a.trim().parse().expect("Please type a number!");

            if a > M {
                M = a;
            }
            if a < m {
                m = a;
            }
            c += a;
        }

        println!("{} {} {}", m, M, c);
    }
