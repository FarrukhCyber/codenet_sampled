use std::io;

    const N: usize = 200;
    const SHIRO: i32 = 0;
    const HAI: i32 = 1;
    const KURO: i32 = 2;

    fn aaaaa(n: usize, a: &mut [[i32; N]; N], color: &mut [i32; N], d: &mut [i32; N], f: &mut [i32; N]) {
        let mut tt = 0;
        for i in 0..n {
            color[i] = SHIRO;
        }
        for u in 0..n {
            if color[u] == SHIRO {
                bbbbb(n, u, a, color, d, f, &mut tt);
            }
        }
        for i in 0..n {
            println!("{} {} {}", i + 1, d[i], f[i]);
        }
    }

    fn bbbbb(n: usize, k: usize, a: &mut [[i32; N]; N], color: &mut [i32; N], d: &mut [i32; N], f: &mut [i32; N], tt: &mut i32) {
        color[k] = HAI;
        *tt += 1;
        d[k] = *tt;
        for v in 0..n {
            if a[k][v] == 0 {
                continue;
            }
            if color[v] == SHIRO {
                bbbbb(n, v, a, color, d, f, tt);
            }
        }
        color[k] = KURO;
        *tt += 1;
        f[k] = *tt;
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = [[0; N]; N];
        let mut color = [0; N];
        let mut d = [0; N];
        let mut f = [0; N];

        for i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let u: usize = iter.next().unwrap().parse().unwrap() - 1;
            let k: usize = iter.next().unwrap().parse().unwrap();
            for j in 0..k {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let v: usize = input.trim().parse().unwrap() - 1;
                a[u][v] = 1;
            }
        }

        aaaaa(n, &mut a, &mut color, &mut d, &mut f);
    }
