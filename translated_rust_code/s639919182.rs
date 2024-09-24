use std::io;

    fn main() {
        let mut v = [0; 10];
        let mut sum = 0;

        for i in 0..10 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            v[i] = input.trim().parse().expect("Please type a number!");
        }

        for i in 0..10 {
            sum += v[i];
        }

        println!("{}", sum);
    }
