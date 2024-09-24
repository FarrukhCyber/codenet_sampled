use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        let mut sum = 0;
        let mut m = n;
        for _ in 0..9 {
            sum += m % 10;
            m /= 10;
        }

        if m % sum == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
