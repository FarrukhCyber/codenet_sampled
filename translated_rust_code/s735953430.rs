use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let n: usize = iter.next().unwrap().parse().unwrap();

        if n == 0 {
            println!("{}", x);
            return;
        }

        let mut plus = vec![0; 256];
        let mut minus = vec![0; 256];
        for i in 0..256 {
            plus[i] = i as i32;
            minus[i] = -(i as i32);
        }

        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let j: usize = input.trim().parse().unwrap();
            plus[j] = 0;
        }

        let mut i_min = 0;
        let mut min = 1000;
        for i in (0..256).rev() {
            if plus[i] == 0 && i > 0 {
                continue;
            }
            let mut k = x - plus[i];
            if k < 0 {
                k *= -1;
            }
            if k <= min {
                min = k;
                i_min = i;
            }
        }

        println!("{}", i_min);
    }
