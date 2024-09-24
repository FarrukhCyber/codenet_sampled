use std::io;

    fn you(mut x: i32, mut y: i32) -> i32 {
        let mut z;
        loop {
            z = x % y;
            x = y;
            y = z;
            if y <= 0 {
                break;
            }
        }
        x
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut numbers = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let mut x = numbers.next().unwrap();
        let mut y = numbers.next().unwrap();
        let z = if x >= y {
            you(x, y)
        } else {
            you(y, x)
        };
        println!("{}", z);
    }
