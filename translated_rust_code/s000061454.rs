use std::io;

    fn main() {
        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
            let (mut a, mut b, n) = (iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap());
            a -= a / b * b;
            let mut sum = 0;
            for _ in 1..=n {
                a *= 10;
                let mut j = 9;
                while b * j > a {
                    j -= 1;
                }
                a -= b * j;
                sum += j;
            }
            println!("{}", sum);
        }
    }
