use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut min = i64::MAX;
        let mut max = i64::MIN;
        let mut sum = 0;

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let tmp: i64 = input.trim().parse().unwrap();

            if min > tmp {
                min = tmp;
            }
            if max < tmp {
                max = tmp;
            }
            sum += tmp;
        }

        println!("{} {} {}", min, max, sum);
    }
