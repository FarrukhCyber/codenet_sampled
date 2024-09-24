use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let dimensions: Vec<_> = input.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
        let (N, D) = (dimensions[0], dimensions[1]);

        let mut matrix = vec![vec![0; D]; N];

        for i in 0..N {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let row: Vec<_> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            matrix[i] = row;
        }

        let mut k = 0;
        for i in 0..N - 1 {
            for j in i + 1..N {
                let mut m = 0;
                for n in 0..D {
                    m += (matrix[i][n] - matrix[j][n]).powi(2);
                }
                let l = (m as f64).sqrt();
                if l.floor() == l.ceil() {
                    k += 1;
                }
            }
        }

        println!("{}", k);
    }
