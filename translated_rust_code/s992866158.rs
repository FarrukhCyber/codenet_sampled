use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();

        let mut cnt = 0;
        for _ in 0..n {
            let mut h = String::new();
            io::stdin().read_line(&mut h).unwrap();
            let h: i32 = h.trim().parse().unwrap();

            if h >= k {
                cnt += 1;
            }
        }

        println!("{}", cnt);
    }
