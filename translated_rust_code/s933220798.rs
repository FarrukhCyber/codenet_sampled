use std::io;

    fn main() {
        let mut ave = 0.0;

        for _ in 0..5 {
            let mut a = String::new();
            io::stdin().read_line(&mut a).expect("Failed to read line");
            let a: f64 = a.trim().parse().expect("Please type a number!");

            if a <= 40.0 {
                ave += 40.0;
            } else {
                ave += a;
            }
        }

        ave /= 5.0;
        println!("{}", ave);
    }
