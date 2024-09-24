use std::io;

    fn chmin(a: i32, b: i32) -> i32 {
        if a > b {
            b
        } else {
            a
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut dp = vec![10000; n];
        let mut h = vec![0; n];

        for i in 0..n {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            h[i] = input.trim().parse().unwrap();
        }

        dp[0] = 0;
        for i in 1..n {
            if i == 1 {
                dp[1] = (h[1] - h[0]).abs();
            } else {
                dp[i] = chmin(dp[i - 1] + (h[i] - h[i - 1]).abs(), dp[i - 2] + (h[i] - h[i - 2]).abs());
            }
        }

        println!("{}", dp[n - 1]);
    }
