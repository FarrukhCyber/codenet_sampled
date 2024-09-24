use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let numbers: Vec<i32> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let (a, b, c, d) = (numbers[0], numbers[1], numbers[2], numbers[3]);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let mut sum = 0;
                    let mut symbols = vec![' ', ' ', ' '];

                    if i == 0 {
                        sum = a + b;
                        symbols[0] = '+';
                    } else {
                        sum = a - b;
                        symbols[0] = '-';
                    }

                    if j == 0 {
                        sum += c;
                        symbols[1] = '+';
                    } else {
                        sum -= c;
                        symbols[1] = '-';
                    }

                    if k == 0 {
                        sum += d;
                        symbols[2] = '+';
                    } else {
                        sum -= d;
                        symbols[2] = '-';
                    }

                    if sum == 7 {
                        println!("{} {} {} {} {} {} = 7", a, symbols[0], b, symbols[1], c, symbols[2], d);
                        return;
                    }
                }
            }
        }
    }
