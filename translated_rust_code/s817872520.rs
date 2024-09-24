use std::io::{self, BufRead};

    const INF: i32 = 0x5fffffff;

    fn getint(p: &mut &[u8]) -> i32 {
        let mut n = 0;
        if p[0] == b'-' {
            *p = &p[1..];
            while p[0] >= b'0' {
                n = (n << 3) + (n << 1) + (p[0] & 0xf) as i32;
                *p = &p[1..];
            }
            return -n;
        }
        while p[0] >= b'0' {
            n = (n << 3) + (n << 1) + (p[0] & 0xf) as i32;
            *p = &p[1..];
        }
        n
    }

    fn main() {
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();

        let w = getint(&mut lines.next().unwrap().unwrap().as_bytes());
        let mut buf = lines.next().unwrap().unwrap().into_bytes();
        let mut p = &mut buf[..];

        let mut a = vec![0; w as usize + 2];
        let mut b = vec![0; w as usize + 2];
        let mut ans = 0;
        let mut f = 0;

        for i in 1..=w {
            a[i as usize] = getint(&mut p);
            p = &p[1..];
            if a[i as usize] == 0 {
                f |= 1;
            } else if a[i as usize] < 0 {
                f |= 2;
            } else {
                ans += a[i as usize];
            }
        }

        if ans == 0 || (f & 1) == 0 {
            println!("0");
            return;
        }

        if (f & 2) == 0 {
            println!("{}", ans);
            return;
        }

        let mut close = 0;
        f = 0;

        for i in 1..=w {
            if a[i as usize] == 0 {
                close = INF;
                f = -1;
            } else if a[i as usize] < 0 {
                if f < 0 || close - (i - f) > -a[i as usize] {
                    close = -a[i as usize];
                    f = i;
                }
            } else {
                if f < 0 {
                    b[i as usize] = INF;
                } else {
                    let k = close - (i - f);
                    b[i as usize] = if k < 0 { 0 } else { k };
                }
            }
        }

        close = 0;
        f = w;

        for i in (1..=w).rev() {
            if a[i as usize] == 0 {
                close = INF;
                f = -1;
            } else if a[i as usize] < 0 {
                if f < 0 || close - (f - i) > -a[i as usize] {
                    close = -a[i as usize];
                    f = i;
                }
            } else {
                if f < 0 {
                    b[i as usize] = INF;
                } else {
                    let k = close - (f - i);
                    if k < 0 {
                        b[i as usize] = 0;
                    } else if k > b[i as usize] {
                        b[i as usize] = k;
                    }
                }
            }
        }

        for i in 1..=w {
            if a[i as usize] > 0 {
                if b[i as usize] > a[i as usize] {
                    ans += a[i as usize];
                } else {
                    ans += b[i as usize];
                }
            }
        }

        println!("{}", ans);
    }
