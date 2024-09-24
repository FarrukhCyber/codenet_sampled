use std::io;

    const MAX: usize = 100;
    const INF: i32 = 999999;
    const NIL: i32 = -1;
    const WHITE: i32 = 0;
    const BLACK: i32 = 2;

    fn prim(n: usize, g: [[i32; MAX]; MAX]) -> i32 {
        let mut d = vec![INF; n];
        let mut pi = vec![NIL; n];
        let mut color = vec![WHITE; n];

        d[0] = 0;
        loop {
            let mut mincost = INF;
            let mut u = 0;

            for i in 0..n {
                if color[i] != BLACK && d[i] < mincost {
                    mincost = d[i];
                    u = i;
                }
            }

            if mincost == INF {
                break;
            }

            color[u] = BLACK;

            for v in 0..n {
                if color[v] != BLACK && g[u][v] < d[v] && g[u][v] != -1 {
                    pi[v] = u as i32;
                    d[v] = g[u][v];
                }
            }
        }

        let mut sum = 0;
        for i in 0..n {
            if d[i] != INF {
                sum += d[i];
            }
        }

        sum
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut g = [[0; MAX]; MAX];
        for i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let row: Vec<i32> = input
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for j in 0..n {
                g[i][j] = row[j];
            }
        }

        println!("{}", prim(n, g));
    }
