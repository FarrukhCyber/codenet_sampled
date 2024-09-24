use std::io;

    const N: usize = 1000000;
    const Q: usize = 50000;

    fn binary_search(key: i32, s: &[i32], n: usize) -> bool {
        let (mut left, mut right) = (0, n);
        while left < right {
            let mid = (left + right) / 2;
            if s[mid] == key {
                return true;
            } else if key < s[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        false
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut s = vec![0; N];
        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            s[i] = input.trim().parse().unwrap();
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let q: usize = input.trim().parse().unwrap();

        let mut t = vec![0; Q];
        let mut c = 0;
        for i in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            t[i] = input.trim().parse().unwrap();
            if binary_search(t[i], &s, n) {
                c += 1;
            }
        }

        println!("{}", c);
    }
