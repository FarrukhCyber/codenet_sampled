use std::io;

    fn main() {
        let mut input = String::new();

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut x = 0;
        let mut kosu = 0;

        io::stdin().read_line(&mut input).expect("Failed to read line");
        a = input.trim().parse().expect("Please type a number!");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        b = input.trim().parse().expect("Please type a number!");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        c = input.trim().parse().expect("Please type a number!");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        x = input.trim().parse().expect("Please type a number!");

        x = x / 50;

        for i in 0..=a {
            for j in 0..=b {
                for k in 0..=c {
                    if x == (10 * i) + (2 * j) + k {
                        kosu += 1;
                    }
                }
            }
        }

        println!("{}", kosu);
    }
