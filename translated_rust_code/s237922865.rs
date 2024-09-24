use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let mut cnt4 = 0;
        let mut cnt2 = 0;

        for _ in 0..n {
            let mut wk = String::new();
            io::stdin().read_line(&mut wk).unwrap();
            let wk: i32 = wk.trim().parse().unwrap();

            if wk % 4 == 0 {
                cnt4 += 1;
            } else if wk % 2 == 0 {
                cnt2 += 1;
            }
        }

        if cnt4 + cnt2 / 2 >= n / 2 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
