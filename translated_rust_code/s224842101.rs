use std::io;

    fn main() {
        let mut count = [0; 4];
        let mut s = 1;

        for _ in 0..3 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let numbers: Vec<_> = input.split_whitespace().map(|s| s.trim().parse::<usize>().unwrap()).collect();
            let a = numbers[0] - 1;
            let b = numbers[1] - 1;

            count[a] += 1;
            count[b] += 1;
        }

        for i in 0..4 {
            s *= count[i];
        }

        if s == 4 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
