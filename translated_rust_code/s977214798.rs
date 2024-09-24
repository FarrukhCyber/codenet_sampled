use std::io::{self, BufRead};

    fn nextint() -> i32 {
        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    fn p(a: i32, b: i32, n: i32) -> i32 {
        if a < b {
            a + b * n
        } else {
            b + a * n
        }
    }

    fn f(from: usize, e1: &[usize], e2: &[usize], d: &mut [i32], b: &mut [bool]) -> i32 {
        if b[from] {
            return -1;
        }
        if d[from] != 0 {
            return d[from];
        }
        b[from] = true;
        let to = e1[from];
        let mut max = 0;
        if to != 0 {
            let now = f(to, e1, e2, d, b);
            if now < 0 {
                return now;
            }
            if now > max {
                max = now;
            }
        }
        let to = e2[from];
        if to != 0 {
            let now = f(to, e1, e2, d, b);
            if now < 0 {
                return now;
            }
            if now > max {
                max = now;
            }
        }
        b[from] = false;
        d[from] = max + 1;
        d[from]
    }

    fn main() {
        let n = nextint() as usize;
        let mut a = vec![vec![0; n - 1]; n];
        let mut e1 = vec![0; 1_000_000];
        let mut e2 = vec![0; 1_000_000];
        let mut d = vec![0; 1_000_000];
        let mut b = vec![false; 1_000_000];

        for i in 0..n {
            for j in 0..n - 1 {
                a[i][j] = nextint() as usize - 1;
            }
        }

        for i in 0..n {
            for j in 0..n - 2 {
                let from = p(i as i32, a[i][j] as i32, n as i32) as usize;
                let to = p(i as i32, a[i][j + 1] as i32, n as i32) as usize;
                if e1[from] == 0 {
                    e1[from] = to;
                } else if e2[from] == 0 {
                    e2[from] = to;
                } else {
                    println!("-1");
                    return;
                }
            }
        }

        let mut ans = 0;
        for i in 0..n {
            let from = p(i as i32, a[i][0] as i32, n as i32) as usize;
            let now = f(from, &e1, &e2, &mut d, &mut b);
            if now < 0 {
                ans = -1;
                break;
            }
            if now > ans {
                ans = now;
            }
        }
        println!("{}", ans);
    }
