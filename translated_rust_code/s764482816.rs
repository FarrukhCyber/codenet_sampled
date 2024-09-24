use std::io;

    fn main() {
        let mut io: [i32; 5] = [0; 5];
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().expect("Please type a number!")).collect();

        for i in 0..5 {
            io[i] = numbers[i];
        }

        for i in 0..5 {
            let (max, max_j) = io[i..].iter().enumerate().fold((io[i], i), |(max, max_j), (j, &val)| if val > max { (val, j) } else { (max, max_j) });
            io.swap(i, max_j + i);
            if i < 4 {
                print!("{} ", io[i]);
            } else {
                println!("{}", io[i]);
            }
        }
    }
