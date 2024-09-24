use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: usize = n.trim().parse().expect("Please type a number!");

        let mut data = Vec::with_capacity(n);
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = input.trim().parse().expect("Please type a number!");
            data.push(input);
        }

        let (max, index) = data.iter().enumerate()
            .fold((i32::MIN, 0), |(max, index), (i, &value)| {
                if value > max {
                    (value, i)
                } else {
                    (max, index)
                }
            });

        let a: i32 = data.iter().enumerate()
            .filter(|&(i, _)| i != index)
            .map(|(_, &value)| value)
            .sum();

        if a > max {
            println!("Yes");
        } else {
            println!("No");
        }
    }
