use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = Vec::new();
        let mut b = Vec::new();

        for _ in 0..n + 1 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            a.push(input.trim().parse().unwrap());
        }

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            b.push(input.trim().parse().unwrap());
        }

        let mut ans = 0;

        for i in 0..n {
            if a[i] >= b[i] {
                ans += b[i];
                a[i] -= b[i];
                b[i] = 0;
            } else {
                ans += a[i];
                b[i] -= a[i];
                a[i] = 0;
            }

            if a[i + 1] >= b[i] {
                ans += b[i];
                a[i + 1] -= b[i];
                b[i] = 0;
            } else {
                ans += a[i + 1];
                b[i] -= a[i + 1];
                a[i + 1] = 0;
            }
        }

        println!("{}", ans);
    }
