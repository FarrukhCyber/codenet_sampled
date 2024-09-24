use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().expect("Please type a number!")).collect();

        let a = numbers[0];
        let b = numbers[1];

        let c = a + b;
        let d = a - b;
        let e = a * b;

        let mut max = c;

        if max < d {
            max = d;
        }

        if max < e {
            max = e;
        }

        println!("{}", max);
    }
