use std::io;

    const M: usize = 1046527;
    const L: usize = 14;

    fn get_char(ch: char) -> usize {
        match ch {
            'A' => 1,
            'C' => 2,
            'G' => 3,
            'T' => 4,
            _ => 0,
        }
    }

    fn get_key(str: &str) -> usize {
        let mut sum = 0;
        let mut p = 1;
        for ch in str.chars() {
            sum += p * get_char(ch);
            p *= 5;
        }
        sum
    }

    fn h1(key: usize) -> usize {
        key % M
    }

    fn h2(key: usize) -> usize {
        1 + (key % (M - 1))
    }

    fn find(str: &str, h: &[String]) -> bool {
        let key = get_key(str);
        let mut i = 0;
        loop {
            let hash = (h1(key) + i * h2(key)) % M;
            if h[hash] == str {
                return true;
            } else if h[hash].is_empty() {
                return false;
            }
            i += 1;
        }
    }

    fn insert(str: &str, h: &mut [String]) -> bool {
        let key = get_key(str);
        let mut i = 0;
        loop {
            let hash = (h1(key) + i * h2(key)) % M;
            if h[hash] == str {
                return true;
            } else if h[hash].is_empty() {
                h[hash] = str.to_string();
                return false;
            }
            i += 1;
        }
    }

    fn main() {
        let mut h = vec![String::new(); M];
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let com = parts.next().unwrap();
            let str = parts.next().unwrap();

            if com == "i" {
                insert(str, &mut h);
            } else {
                if find(str, &h) {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
        }
    }
