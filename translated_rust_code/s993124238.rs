use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let dimensions: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = dimensions[0];
        let d = dimensions[1];

        let mut points = vec![vec![0; d + 1]; n + 1];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let point_values: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
            for j in 1..=d {
                points[i][j] = point_values[j - 1];
            }
        }

        let mut count = 0;
        for i in 1..=n - 1 {
            for j in i + 1..=n {
                let mut sum = 0;
                for k in 1..=d {
                    sum += (points[i][k] - points[j][k]).powi(2);
                }
                let distance = (sum as f64).sqrt();
                if distance.floor() == distance {
                    count += 1;
                }
            }
        }

        println!("{}", count);
    }
