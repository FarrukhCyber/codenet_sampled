use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut numbers = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        let a = numbers.next().unwrap();
        let b = numbers.next().unwrap();
        let c = numbers.next().unwrap();

        if b <= c {
            println!("NO");
            return;
        }

        let mut d = a;
        let e = d % b;

        for i in 0..b {
            d = d % b;
            if d == c {
                println!("YES");
                return;
            } else if i != 0 && d == c {
                println!("NO");
                return;
            }
            d += a;
        }

        println!("NO");
    }
