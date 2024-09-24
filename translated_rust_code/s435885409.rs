use std::io;

    fn calc_score() -> i32 {
        let mut out = 0;
        let mut score = 0;
        let mut base = 0;

        while out < 3 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();

            if input == "OUT" {
                out += 1;
            } else if input == "HIT" {
                if base == 3 {
                    score += 1;
                } else {
                    base += 1;
                }
            } else if input == "HOMERUN" {
                score += base + 1;
                base = 0;
            }
        }

        score
    }

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i32 = n.trim().parse().expect("Please type a number!");

        for _ in 0..n {
            println!("{}", calc_score());
        }
    }
