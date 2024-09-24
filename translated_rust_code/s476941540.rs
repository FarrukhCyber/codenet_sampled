use std::io;

    fn main() {
        let mut alpha = [0; 26];

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            if input.trim().is_empty() {
                break;
            }

            if let Some(ch) = input.trim().chars().next() {
                if ch.is_alphabetic() {
                    let index = if ch.is_uppercase() {
                        ch as usize - 'A' as usize
                    } else {
                        ch as usize - 'a' as usize
                    };
                    alpha[index] += 1;
                }
            }
        }

        let mut ch = 'a';
        for i in 0..26 {
            println!("{} : {}", ch, alpha[i]);
            ch = (ch as u8 + 1) as char;
        }
    }
