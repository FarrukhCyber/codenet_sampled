use std::io;

    const LEN: usize = 300;
    const TRUE: i32 = 1;
    const FALSE: i32 = -1;

    fn solve(i: isize, m: i32, a: &[i32], flag: &mut i32) -> i32 {
        if i >= n as isize || m < 0 {
            return FALSE;
        } else if m == 0 || m - a[(i + 1) as usize] == 0 {
            *flag = TRUE;
            return TRUE;
        } else {
            // not use a[i]
            solve(i + 1, m, a, flag);
            // use a[i]
            solve(i + 1, m - a[i as usize], a, flag);
        }
        FALSE
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = [0; LEN];
        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            a[i] = input.trim().parse().unwrap();
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let q: usize = input.trim().parse().unwrap();

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let m: i32 = input.trim().parse().unwrap();

            let mut flag = FALSE;
            solve(-1, m, &a, &mut flag);

            if flag == TRUE {
                println!("yes");
            } else {
                println!("no");
            }
        }
    }
    python
    # The provided code is already in Rust. No need for translation.
