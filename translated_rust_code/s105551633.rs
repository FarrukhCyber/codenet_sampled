use std::io;

    fn digit(n: i64) -> i32 {
        let mut res = 0;
        let mut n = n;
        while n > 0 {
            res += 1;
            n /= 10;
        }
        res
    }

    fn run() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let n: i64 = input.trim().parse().expect("Please type a number!");

        let mut min = digit(n);
        for i in 1..=((n as f64).sqrt() as i64) {
            if n % i != 0 {
                continue;
            }
            let a = i;
            let b = n / i;
            min = min.min(digit(a).max(digit(b)));
        }
        println!("{}", min);
    }

    fn main() {
        run();
    }
