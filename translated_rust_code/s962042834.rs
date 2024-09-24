use std::io;

    fn kansu(a: usize, b: usize) {
        let mut suu = vec![vec![0; 17]; 17];
        let mut kouji = vec![vec![0; 17]; 17];
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        let n: usize = n.trim().parse().unwrap();

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let coords: Vec<&str> = input.trim().split(' ').collect();
            let x: usize = coords[0].parse().unwrap();
            let y: usize = coords[1].parse().unwrap();
            kouji[x][y] = 1;
        }

        for i in 1..=a {
            for j in 1..=b {
                if kouji[i][j] == 1 {
                    suu[i][j] = 0;
                } else {
                    if i == 1 && j == 1 {
                        suu[i][j] = 1;
                    } else {
                        if i == 1 {
                            suu[i][j] = suu[i][j - 1];
                        } else if j == 1 {
                            suu[i][j] = suu[i - 1][j];
                        } else {
                            suu[i][j] = suu[i][j - 1] + suu[i - 1][j];
                        }
                    }
                }
            }
        }
        println!("{}", suu[a][b]);
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let coords: Vec<&str> = input.trim().split(' ').collect();
        let a: usize = coords[0].parse().unwrap();
        let b: usize = coords[1].parse().unwrap();

        while a != 0 && b != 0 {
            kansu(a, b);
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let coords: Vec<&str> = input.trim().split(' ').collect();
            let a: usize = coords[0].parse().unwrap();
            let b: usize = coords[1].parse().unwrap();
        }
    }
    
    [/INST]
