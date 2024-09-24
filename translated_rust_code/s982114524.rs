use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let mut x = 0;
        for i in 1..=n {
            x += i;
        }

        println!("{}", x);
    }
