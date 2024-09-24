use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i32 = n.trim().parse().expect("Please type a number!");

        for i in 1..=n {
            let mut a = i;
            if a % 3 == 0 {
                print!(" {}", i);
            } else {
                while a > 0 {
                    if a % 10 == 3 {
                        print!(" {}", i);
                        a = 0;
                    }
                    a /= 10;
                }
            }
        }
        println!();
    }
