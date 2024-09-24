use std::io;

    fn main() {
        let mut input1 = String::new();
        let mut input2 = String::new();

        io::stdin().read_line(&mut input1).expect("Failed to read line");
        io::stdin().read_line(&mut input2).expect("Failed to read line");

        let input1 = input1.trim();
        let input2 = input2.trim();

        if input1.len() != input2.len() {
            println!("NO");
        } else {
            let reversed1: String = input1.chars().rev().collect();
            if reversed1 == input2 {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
