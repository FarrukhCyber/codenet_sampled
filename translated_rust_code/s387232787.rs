use std::io;

    fn main() {
        let mut c = [0; 26];
        let mut input = String::new();

        while io::stdin().read_line(&mut input).unwrap() > 0 {
            for ch in input.chars() {
                if ch.is_ascii_lowercase() {
                    c[(ch as u8 - b'a') as usize] += 1;
                } else if ch.is_ascii_uppercase() {
                    c[(ch as u8 - b'A') as usize] += 1;
                }
            }
            input.clear();
        }

        for i in 0..26 {
            println!("{} : {}", (b'a' + i as u8) as char, c[i]);
        }
    }
