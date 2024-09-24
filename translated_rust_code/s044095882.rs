use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = vec![0; n];
        let mut count = vec![0; n];

        for i in 1..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            a[i] = input.trim().parse().unwrap();
        }

        for i in 0..n {
            count[i] = 0;
        }

        for i in 1..n {
            count[a[i] - 1] += 1;
        }

        for i in 0..n {
            println!("{}", count[i]);
        }
    }
