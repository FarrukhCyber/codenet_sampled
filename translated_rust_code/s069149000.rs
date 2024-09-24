use std::io;

    const INF: i32 = 100000;
    const MAX: usize = 101;
    const NIL: i32 = -6;
    const WHITE: i32 = 0;
    const GRAY: i32 = 1;
    const BLACK: i32 = 2;

    fn prim(mut g: [[i32; MAX]; MAX], n: usize) {
        let mut d = [INF; MAX];
        let mut pi = [NIL; MAX];
        let mut color = [WHITE; MAX];
        let mut sum = 0;

        d[0] = 0;

        loop {
            let mut min = INF;
            let mut u = NIL;
            for i in 0..n {
                if color[i] != BLACK && d[i] < min {
                    min = d[i];
                    u = i as i32;
                }
            }
            if u == NIL {
                break;
            }
            color[u as usize] = BLACK;
            for v in 0..n {
                if color[v] != BLACK && g[u as usize][v] != INF {
                    if d[v] > g[u as usize][v] {
                        d[v] = g[u as usize][v];
                        pi[v] = u;
                        color[v] = GRAY;
                    }
                }
            }
        }
        for i in 0..n {
            if pi[i] != NIL {
                sum += g[i][pi[i] as usize];
            }
        }
        println!("{}", sum);
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut g = [[0; MAX]; MAX];
        for i in 0..n {
            for j in 0..n {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let e: i32 = input.trim().parse().unwrap();
                g[i][j] = if e == -1 { INF } else { e };
            }
        }

        prim(g, n);
    }
