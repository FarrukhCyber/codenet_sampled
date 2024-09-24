use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n = n.trim();
        let chars: Vec<char> = n.chars().collect();

        if ((chars[0] == chars[1] && chars[1] == chars[2]) || (chars[1] == chars[2] && chars[2] == chars[3])) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
