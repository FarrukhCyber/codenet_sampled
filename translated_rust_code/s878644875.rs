use std::io;

    fn main() {
        let mut s = String::new();
        let mut t = String::new();

        io::stdin().read_line(&mut s).expect("Failed to read line");
        io::stdin().read_line(&mut t).expect("Failed to read line");

        let s = s.trim();
        let t = t.trim();

        let mut hit = 0;
        for i in 0..3 {
            if s.chars().nth(i) == t.chars().nth(i) {
                hit += 1;
            }
        }

        println!("{}", hit);
    }
