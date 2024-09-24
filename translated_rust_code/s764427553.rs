use std::io;

    const MAX_V: usize = 999999;

    fn main() {
        let mut prime = vec![true; MAX_V + 1];
        for i in 2..=MAX_V {
            if prime[i] {
                let mut k = 2 * i;
                while k <= MAX_V {
                    prime[k] = false;
                    k += i;
                }
            }
        }
        prime[0] = false;
        prime[1] = false;

        let mut count = 0;
        let mut input = String::new();
        while io::stdin().read_line(&mut input).unwrap() > 0 {
            count += 1;
            let n: usize = input.trim().parse().unwrap();
            let cnt = (1..=n).filter(|&i| prime[i]).count();
            println!("{}", cnt);
            if count == 30 {
                break;
            }
            input.clear();
        }
    }
