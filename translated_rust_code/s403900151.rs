use std::io;

    fn main() {
        let mut input = String::new();

        loop {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let n: i32 = iter.next().unwrap().parse().unwrap();
            let r: i32 = iter.next().unwrap().parse().unwrap();

            if n == 0 && r == 0 {
                break;
            }

            let mut a: Vec<i32> = (1..=n).collect();
            let mut b: Vec<i32> = vec![0; n as usize];

            for _ in 0..r {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();
                let mut iter = input.split_whitespace();
                let p: usize = iter.next().unwrap().parse().unwrap();
                let c: usize = iter.next().unwrap().parse().unwrap();

                for j in 0..c {
                    b[j] = a[n as usize - p - c + 1 + j];
                }

                for j in 0..p - 1 {
                    a[n as usize - p - c + 1 + j] = a[n as usize - p + j + 1];
                }

                for (j, k) in (n as usize - p - c + 1 + (0..c)).enumerate() {
                    a[k] = b[j];
                }
            }

            println!("{}", a[n as usize - 1]);
        }
    }
