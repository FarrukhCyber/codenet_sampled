use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut output = String::new();

        for c in input.trim().chars() {
            if c != 'B' {
                output.push(c);
            } else if !output.is_empty() {
                output.pop();
            }
        }

        println!("{}", output);
    }
