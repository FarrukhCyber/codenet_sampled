use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: usize = n.trim().parse().expect("Please type a number!");

        let mut a = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = input.trim().parse().expect("Please type a number!");
            a.push(input);
        }

        let mut sum = 0;
        for i in 1..n {
            if a[i - 1] > a[i] {
                sum += a[i - 1] - a[i];
                a[i] = a[i - 1];
            }
        }

        println!("{}", sum);
    }
