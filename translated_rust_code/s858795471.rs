use std::io;

    fn search(a: &[i32], n: usize, key: i32) -> bool {
        let mut i = 0;
        while a[i] != key {
            i += 1;
            if i == n {
                return false;
            }
        }
        true
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = vec![0; n + 1];
        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            a[i] = input.trim().parse().unwrap();
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let q: usize = input.trim().parse().unwrap();

        let mut sum = 0;
        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let key: i32 = input.trim().parse().unwrap();
            if search(&a, n, key) {
                sum += 1;
            }
        }

        println!("{}", sum);
    }
