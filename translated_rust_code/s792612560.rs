use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: u64 = input.trim().parse().unwrap();

        let mut m = n;
        let mut n = n - 1;

        while n > 1 {
            m *= n;
            n -= 1;
        }

        println!("{}", m);
    }
