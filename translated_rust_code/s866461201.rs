use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i32 = n.trim().parse().expect("Please type a number!");

        let mut fd = [0; 6];

        for _ in 0..n {
            let mut height = String::new();
            io::stdin().read_line(&mut height).expect("Failed to read line");
            let height: f64 = height.trim().parse().expect("Please type a number!");

            if height >= 185.0 {
                fd[5] += 1;
            } else if height >= 180.0 {
                fd[4] += 1;
            } else if height >= 175.0 {
                fd[3] += 1;
            } else if height >= 170.0 {
                fd[2] += 1;
            } else if height >= 165.0 {
                fd[1] += 1;
            } else {
                fd[0] += 1;
            }
        }

        for i in 0..6 {
            print!("{}:", i + 1);
            for _ in 0..fd[i] {
                print!("*");
            }
            println!();
        }
    }
