use std::io;

    fn main() {
        let mut input = String::new();
        loop {
            input.clear();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut iter = input.split_whitespace();
            let a: i32 = match iter.next().and_then(|s| s.parse().ok()) {
                Some(num) => num,
                None => break,
            };
            let b = iter.next().unwrap().parse::<i32>().unwrap();
            let c = iter.next().unwrap().parse::<i32>().unwrap();
            let d = iter.next().unwrap().parse::<i32>().unwrap();
            let e = iter.next().unwrap().parse::<i32>().unwrap();
            let f = iter.next().unwrap().parse::<i32>().unwrap();

            let y = (c as f64 * d as f64 - f as f64 * a as f64) / (b as f64 * d as f64 - e as f64 * a as f64);
            let x = (c as f64 * e as f64 - f as f64 * b as f64) / (a as f64 * e as f64 - d as f64 * b as f64);

            let y = if y <= 0.0 && y > -0.0005 { 0.0 } else { y };
            let x = if x <= 0.0 && x > -0.0005 { 0.0 } else { x };

            println!("{:.3} {:.3}", x, y);
        }
    }
