use std::io;

    fn main() {
        let mut a = [0; 1000];
        let mut n = String::new();
        let mut d = 0;

        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: usize = n.trim().parse().expect("Please type a number!");

        for i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            a[i] = input.trim().parse().expect("Please type a number!");
        }

        for i in 0..n {
            for j in (i+1..n).rev() {
                if a[j] < a[j-1] {
                    a.swap(j, j-1);
                    d += 1;
                }
            }
        }

        print!("{}", a[0]);
        for i in 1..n {
            print!(" {}", a[i]);
        }
        println!("\n{}", d);
    }
