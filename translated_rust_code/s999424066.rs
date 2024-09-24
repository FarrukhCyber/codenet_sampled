use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());

        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let mut c = iter.next().unwrap();
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();

        c *= 2;

        if a + b <= c {
            println!("{}", a * x + b * y);
        } else {
            if x <= y {
                if b <= c {
                    println!("{}", c * x + (y - x) * b);
                } else {
                    println!("{}", c * y);
                }
            } else {
                if a <= c {
                    println!("{}", c * y + (x - y) * a);
                } else {
                    println!("{}", c * x);
                }
            }
        }
    }
