use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: i32 = iter.next().unwrap().trim().parse().unwrap();
        let d: i32 = iter.next().unwrap().trim().parse().unwrap();

        let mut ans = n / (d * 2 + 1);
        let rem = n % (d * 2 + 1);

        if rem != 0 {
            ans += 1;
        }

        println!("{}", ans);
    }
