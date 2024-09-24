use std::io;

    fn main() {
        let mut c = [0; 256];
        let mut i = 0;

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut iter = input.split_whitespace();
            let a: i32 = iter.next().unwrap().parse().unwrap();
            let m: char = iter.next().unwrap().chars().next().unwrap();
            let b: i32 = iter.next().unwrap().parse().unwrap();

            if m == '?' {
                break;
            }

            match m {
                '+' => c[i] = a + b,
                '-' => c[i] = a - b,
                '*' => c[i] = a * b,
                '/' => c[i] = a / b,
                _ => {}
            }

            i += 1;
        }

        for j in 0..i {
            println!("{}", c[j]);
        }
    }
