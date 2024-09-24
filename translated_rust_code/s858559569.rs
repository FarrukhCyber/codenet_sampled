use std::io;

    const N: usize = 4;

    #[derive(Clone, Copy)]
    struct Pazz {
        p: [[i32; N]; N],
    }

    fn heu(q: &Pazz) -> i32 {
        let mut sum = 0;
        for i in 0..N {
            for j in 0..N {
                if q.p[i][j] == 0 {
                    continue;
                }

                let hx = (q.p[i][j] - 1) % N as i32 - j as i32;
                let hy = (q.p[i][j] - 1) / N as i32 - i as i32;

                sum += if hx < 0 { -hx } else { hx };
                sum += if hy < 0 { -hy } else { hy };
            }
        }
        sum
    }

    fn dfs(x: usize, y: usize, max: i32, count: i32, n: isize, q: &mut Pazz) -> i32 {
        let dx = [0, 1, 0, -1];
        let dy = [1, 0, -1, 0];

        let h = heu(q);

        if h == 0 {
            return count;
        }
        if count + h > max {
            return -1;
        }

        for i in 0..4 {
            let x0 = x as isize + dx[i];
            let y0 = y as isize + dy[i];

            if ((i == (n + 2) % 4 && n != -1)
                || !(0 <= x0 && x0 < N as isize && 0 <= y0 && y0 < N as isize))
                || x0 < 0
                || y0 < 0
            {
                continue;
            }

            let x0 = x0 as usize;
            let y0 = y0 as usize;

            let temp = q.p[y][x];
            q.p[y][x] = q.p[y0][x0];
            q.p[y0][x0] = temp;

            let ans = dfs(x0, y0, max, count + 1, i as isize, q);

            if ans != -1 {
                return ans;
            }

            q.p[y][x] = q.p[y0][x0];
            q.p[y0][x0] = temp;
        }

        -1
    }

    fn main() {
        let mut q = Pazz { p: [[0; N]; N] };
        let mut kx = 0;
        let mut ky = 0;

        for i in 0..N {
            for j in 0..N {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let a: i32 = input.trim().parse().unwrap();
                q.p[i][j] = a;
                if a == 0 {
                    kx = j;
                    ky = i;
                }
            }
        }

        let tp = q;
        for i in 1..46 {
            q = tp;
            let ans = dfs(kx, ky, i, 0, -1, &mut q);
            if ans != -1 {
                println!("{}", ans);
                break;
            }
        }
    }
