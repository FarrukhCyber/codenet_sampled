use std::io;

    const MAX: usize = 101;
    const WHITE: i32 = 0;
    const BLACK: i32 = 2;
    const GRAY: i32 = 1;

    fn visit(u: usize, time: &mut i32, g: &[[i32; MAX]; MAX], d: &mut [i32; MAX], f: &mut [i32; MAX], color: &mut [i32; MAX]) {
        color[u] = GRAY;
        *time += 1;
        d[u] = *time;

        for i in 0..n {
            if g[u][i] == 0 {
                continue;
            }
            if color[i] == WHITE {
                visit(i, time, g, d, f, color);
            }
        }

        color[u] = BLACK;
        *time += 1;
        f[u] = *time;
    }

    fn dfs(n: usize, g: &[[i32; MAX]; MAX]) {
        let mut time = 0;
        let mut d = [0; MAX];
        let mut f = [0; MAX];
        let mut color = [WHITE; MAX];

        for i in 0..n {
            if color[i] == WHITE {
                visit(i, &mut time, g, &mut d, &mut f, &mut color);
            }
        }

        for i in 0..n {
            println!("{} {} {}", i + 1, d[i], f[i]);
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut g = [[0; MAX]; MAX];

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let x: usize = iter.next().unwrap().parse().unwrap();
            let y: usize = iter.next().unwrap().parse().unwrap();

            for _ in 0..y {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let z: usize = input.trim().parse().unwrap();
                g[x - 1][z - 1] = 1;
            }
        }

        dfs(n, &g);
    }
