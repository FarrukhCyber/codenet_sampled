use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: usize = n.trim().parse().expect("Please type a number!");

        let mut x = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = input.trim().parse().expect("Please type a number!");
            x.push(input);
        }

        let mut q = String::new();
        io::stdin().read_line(&mut q).expect("Failed to read line");
        let q: usize = q.trim().parse().expect("Please type a number!");

        let mut y = Vec::new();
        for _ in 0..q {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = input.trim().parse().expect("Please type a number!");
            y.push(input);
        }

        let mut sum = 0;
        for i in 0..q {
            for j in 0..n {
                if y[i] == x[j] {
                    sum += 1;
                    break;
                }
            }
        }

        println!("{}", sum);
    }
