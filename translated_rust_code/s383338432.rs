use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let dimensions: Vec<_> = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let height = dimensions[0];
        let width = dimensions[1];

        println!("{}", "#".repeat(width + 2));

        for _ in 0..height {
            let mut input_str = String::new();
            io::stdin().read_line(&mut input_str).unwrap();
            println!("#{}#", input_str.trim());
        }

        println!("{}", "#".repeat(width + 2));
    }
