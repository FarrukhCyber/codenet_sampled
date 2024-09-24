use std::io;

    fn main() {
        let mut input = String::new();
        let stdin = io::stdin();

        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let n: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();

            if n == 0 && r == 0 {
                break;
            }

            let mut a: Vec<usize> = (1..=n).rev().collect();
            let mut b: Vec<usize> = vec![0; n];

            for _ in 0..r {
                input.clear();
                stdin.read_line(&mut input).unwrap();
                let mut iter = input.split_whitespace();
                let p: usize = iter.next().unwrap().parse().unwrap();
                let c: usize = iter.next().unwrap().parse().unwrap();

                b.copy_from_slice(&a);

                for j in 0..c {
                    a[j] = b[p - 1 + j];
                }

                for j in c..c + p - 1 {
                    a[j] = b[j - c];
                }
            }

            println!("{}", a[0]);
        }
    }
