use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let words: Vec<&str> = input.trim().split_whitespace().collect();

        if words.len() != 3 {
            println!("NO");
            return;
        }

        let a = words[0];
        let b = words[1];
        let c = words[2];

        if a.ends_with(b.chars().next().unwrap()) && b.ends_with(c.chars().next().unwrap()) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
