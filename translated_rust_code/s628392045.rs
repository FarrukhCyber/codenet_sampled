use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut h = vec![0; n];
        let mut max = -1;
        let mut flag = 0;

        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            h[i] = input.trim().parse().unwrap();

            if max < h[i] {
                max = h[i];
            }

            if max - h[i] >= 2 {
                flag = 1;
            }
        }

        if flag == 1 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
