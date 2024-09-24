use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut s = input.trim().to_string();

            while let Some(pos) = s.find("Hoshino") {
                let mut chars: Vec<char> = s.chars().collect();
                chars[pos + 6] = 'a';
                s = chars.into_iter().collect();
            }

            println!("{}", s);
        }
    }
