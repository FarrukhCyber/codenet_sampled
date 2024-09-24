use std::io;

    fn run() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();

        let mut a = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            a.push(input.trim().parse().unwrap());
        }

        let mut dp = vec![0; k + 1];
        for i in 1..=k {
            let mut win = 0;
            for j in 0..n {
                if a[j] <= i && dp[i - a[j]] == 0 {
                    win = 1;
                }
            }
            dp[i] = win;
        }

        println!("{}", if dp[k] == 1 {"First"} else {"Second"});
    }

    fn main() {
        run();
    }
