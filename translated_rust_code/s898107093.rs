use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut taro = 0;
        let mut hanako = 0;

        for _ in 0..n {
            let mut t = String::new();
            let mut h = String::new();

            io::stdin().read_line(&mut t).unwrap();
            io::stdin().read_line(&mut h).unwrap();

            let t = t.trim();
            let h = h.trim();

            match t.cmp(h) {
                std::cmp::Ordering::Less => hanako += 3,
                std::cmp::Ordering::Greater => taro += 3,
                std::cmp::Ordering::Equal => {
                    taro += 1;
                    hanako += 1;
                }
            }
        }

        println!("{} {}", taro, hanako);
    }
