use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        let mut r0 = vec![false; n];
        let mut r1 = Vec::new();

        for _ in 0..m {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let mut a: usize = iter.next().unwrap().parse().unwrap();
            let mut b: usize = iter.next().unwrap().parse().unwrap();

            if a > b {
                let t = a;
                a = b;
                b = t;
            }

            if a == 1 {
                r0[b - 1] = true;
            }

            if b == n {
                r1.push(a - 1);
            }
        }

        let mut res = false;

        for i in r1 {
            if r0[i] {
                res = true;
                break;
            }
        }

        println!("{}", if res { "POSSIBLE" } else { "IMPOSSIBLE" });
    }
