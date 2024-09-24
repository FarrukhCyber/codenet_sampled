use std::io;

    fn max(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }

    fn main() {
        let mut data = [0; 111111];
        let mut seq = [0; 222222];

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let n: i32 = iter.next().unwrap().parse().unwrap();
            let k: i32 = iter.next().unwrap().parse().unwrap();

            if n == 0 && k == 0 {
                break;
            }

            data.iter_mut().for_each(|x| *x = 0);
            seq.iter_mut().for_each(|x| *x = 0);

            for _ in 0..k {
                let mut t = String::new();
                io::stdin().read_line(&mut t).unwrap();
                let t: usize = t.trim().parse().unwrap();
                data[t] += 1;
            }

            let mut p = 0;
            for i in 1..=n {
                if data[i as usize] > 0 {
                    seq[p] += 1;
                } else {
                    p += 2;
                }
            }
            p += 1;

            let mut ret = 0;
            if data[0] > 0 {
                for i in 0..p {
                    ret = max(ret, seq[i] + seq[i + 2] + 1);
                }
            } else {
                for i in 0..p {
                    ret = max(ret, seq[i]);
                }
            }

            println!("{}", ret);
        }
    }
