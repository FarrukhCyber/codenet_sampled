use std::io;

    fn linear_search(arr: &[i32], key: i32) -> bool {
        arr.contains(&key)
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

        let mut t = Vec::new();
        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            t.push(num);
        }

        let count = t.iter().filter(|&&key| linear_search(&s, key)).count();

        println!("{}", count);
    }
