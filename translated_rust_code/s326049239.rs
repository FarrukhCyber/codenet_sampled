use std::io;

    fn main() {
        let mut input = String::new();
        while io::stdin().read_line(&mut input).is_ok() {
            let a: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => break,
            };
            input.clear();

            if a < 3 {
                println!("0");
            } else {
                let mut b = a;
                let mut c = 0;
                let mut k = 0;

                for _ in 0..a {
                    b -= 1;
                    c += 1;

                    if c == 3 {
                        k += 1;
                        c = 0;
                    }
                }

                println!("{}", k);
            }
        }
    }
