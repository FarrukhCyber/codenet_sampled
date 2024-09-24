use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let dimensions: Vec<_> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (m, n) = (dimensions[0], dimensions[1]);

        let mut A = vec![vec![0; n]; m];
        let mut B = vec![0; n];
        let mut S = vec![0; m];

        for i in 0..m {
            let mut row = String::new();
            io::stdin().read_line(&mut row).unwrap();
            let row_values: Vec<_> = row.split_whitespace().map(|s| s.parse().unwrap()).collect();
            for j in 0..n {
                A[i][j] = row_values[j];
            }
        }

        for i in 0..n {
            let mut value = String::new();
            io::stdin().read_line(&mut value).unwrap();
            B[i] = value.trim().parse().unwrap();
        }

        for i in 0..m {
            for j in 0..n {
                S[i] += A[i][j] * B[j];
            }
        }

        for i in 0..m {
            println!("{}", S[i]);
        }
    }
