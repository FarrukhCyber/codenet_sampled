use std::io;

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a * b / gcd(a, b)
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: i64 = iter.next().unwrap().parse().unwrap();
        let k: i64 = iter.next().unwrap().parse().unwrap();
        let q: i64 = iter.next().unwrap().parse().unwrap();

        let mut point = vec![0; n as usize];
        let mut a = vec![0; q as usize];

        for i in 0..q {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            a[i as usize] = input.trim().parse().unwrap();
            point[(a[i as usize] - 1) as usize] += 1;
        }

        for i in 0..n {
            if k - (q - point[i as usize]) > 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
