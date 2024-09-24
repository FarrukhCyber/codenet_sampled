use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let j: i32 = input.trim().parse().unwrap();

        for _ in 0..j {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let a: i32 = input.trim().parse().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let b: i32 = input.trim().parse().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let c: i32 = input.trim().parse().unwrap();

            let mut max = a;
            let mut k = 0;
            if max <= b {
                max = b;
                k = 1;
            }
            if max <= c {
                max = c;
                k = 2;
            }

            max = max * max;
            let a = a * a;
            let b = b * b;
            let c = c * c;

            if k == 0 && max == b + c {
                println!("YES");
            } else if k == 1 && max == a + c {
                println!("YES");
            } else if k == 2 && max == a + b {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
