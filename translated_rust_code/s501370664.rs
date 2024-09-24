use std::io;

    fn prime(num: usize) -> Vec<bool> {
        let mut flag = vec![false; num + 1];
        for i in 2..=((num as f64).sqrt() as usize) {
            if !flag[i] {
                for j in (i * i..=num).step_by(i) {
                    flag[j] = true;
                }
            }
        }
        flag
    }

    fn main() {
        let flag = prime(50000);
        let mut input = String::new();

        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let n: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            if n == 0 {
                break;
            }

            let mut i = 2;
            let mut cnt = 0;
            while i * 2 <= n {
                if !flag[i] && !flag[n - i] {
                    cnt += 1;
                }
                i += 1;
            }

            println!("{}", cnt);
        }
    }
