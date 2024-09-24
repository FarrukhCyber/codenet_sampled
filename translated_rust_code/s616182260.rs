use std::io;

    fn main() {
        let mut c = [[0; 3]; 3];
        let mut a = [0; 3];
        let mut b = [0; 3];

        for i in 0..3 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().unwrap()).collect();
            c[i] = [numbers[0], numbers[1], numbers[2]];
        }

        for i in 0..3 {
            b[i] = c[0][i];
        }

        for i in 1..3 {
            a[i] = c[i][0] - b[0];
        }

        for i in 0..3 {
            for j in 0..3 {
                if c[i][j] != a[i] + b[j] {
                    println!("No");
                    return;
                }
            }
        }

        println!("Yes");
    }
