use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let mut nt = 0;
        let mut nx = 0;
        let mut ny = 0;

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let t: i32 = iter.next().unwrap().parse().unwrap();
            let x: i32 = iter.next().unwrap().parse().unwrap();
            let y: i32 = iter.next().unwrap().parse().unwrap();

            let tmp = t - nt - ((x - nx).abs() + (y - ny).abs());
            if tmp < 0 || tmp % 2 != 0 {
                println!("No");
                return;
            }

            nt = t;
            nx = x;
            ny = y;
        }

        println!("Yes");
    }
