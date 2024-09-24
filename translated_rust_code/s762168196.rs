use std::io;

    fn main() {
        let mut a = [0; 10000];
        let mut min = 0;
        let mut max = 0;
        let mut n = String::new();
        let mut sum = 0i64;

        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: usize = n.trim().parse().expect("Please type a number!");

        for i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            a[i] = input.trim().parse().expect("Please type a number!");
            sum += a[i] as i64;
        }

        min = a[0];
        max = a[0];

        for i in 1..n {
            if a[i] > max {
                max = a[i];
            }
            if a[i] < min {
                min = a[i];
            }
        }

        println!("{} {} {}", min, max, sum);
    }
