use std::io;

    fn main() {
        let mut x = [0; 10000];
        let mut i = 0;

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        x[i] = input.trim().parse().unwrap();

        while x[i] != 0 {
            i += 1;
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            x[i] = input.trim().parse().unwrap();
        }

        for j in 0..i {
            println!("Case {}: {}", j + 1, x[j]);
        }
    }
