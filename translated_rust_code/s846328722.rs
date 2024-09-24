use std::io;

    fn linear_search(arr: &[i32], key: i32) -> bool {
        let mut i = 0;
        while i < arr.len() && arr[i] != key {
            i += 1;
        }
        i < arr.len()
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut s = Vec::new();
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            s.push(num);
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let q: usize = input.trim().parse().unwrap();

        let mut count = 0;
        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let t: i32 = input.trim().parse().unwrap();
            if linear_search(&s, t) {
                count += 1;
            }
        }

        println!("{}", count);
    }
