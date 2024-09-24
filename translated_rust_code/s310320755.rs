use std::io;

    fn main() {
        let mut arr = [[0; 16]; 16];
        let mut input = String::new();

        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let n: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };

            if n == 0 {
                break;
            }

            let mut flag = 1;
            let mut r = (n + 1) / 2 + 1;
            let mut c = (n + 1) / 2;
            let mut cnt = 1;
            arr[r as usize][c as usize] = cnt;

            while cnt != n * n + 1 {
                cnt += 1;
                if flag == 1 {
                    r += 1;
                    c += 1;
                    if c > n {
                        c = 1;
                    }
                    if r > n {
                        r = 1;
                    }
                    if arr[r as usize][c as usize] != 0 {
                        flag = 0;
                        cnt -= 1;
                    } else {
                        arr[r as usize][c as usize] = cnt;
                    }
                } else {
                    r += 1;
                    c -= 1;
                    if c < 1 {
                        c = n;
                    }
                    if r > n {
                        r = 1;
                    }
                    if arr[r as usize][c as usize] != 0 {
                        flag = 0;
                    } else {
                        arr[r as usize][c as usize] = cnt;
                        flag = 1;
                    }
                }
            }

            for i in 1..=n {
                for j in 1..=n {
                    print!("{:4}", arr[i as usize][j as usize]);
                    arr[i as usize][j as usize] = 0;
                    if j == n {
                        println!();
                    }
                }
            }
        }
    }
