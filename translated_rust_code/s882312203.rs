use std::io;

    fn main() {
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let chars: Vec<char> = input.trim().chars().collect();

        for (i, &c1) in chars.iter().enumerate() {
            for (&c2, j) in chars.iter().enumerate() {
                if i != j && c1 == c2 {
                    println!("no");
                    return;
                }
            }
        }

        println!("yes");
    }
