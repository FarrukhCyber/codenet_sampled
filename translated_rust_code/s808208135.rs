use std::io;

    const N: usize = 100000;
    const INF: i32 = 1000000;

    struct Date {
        go: usize,
        back: usize,
        cost: i32,
    }

    fn root(a: &[Date], d: &mut [i32]) {
        let n = d.len();
        for i in 0..n {
            d[i] = INF;
        }
        d[0] = 0;

        let mut flag = 1;
        while flag != 0 {
            flag = 0;
            for i in 0..a.len() {
                if a[i].cost + d[a[i].back] < d[a[i].go] {
                    d[a[i].go] = a[i].cost + d[a[i].back];
                    flag = 1;
                }
            }
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = Vec::new();
        let mut d = vec![0; N];
        let mut count = 0;

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let u: usize = iter.next().unwrap().parse().unwrap();
            let k: usize = iter.next().unwrap().parse().unwrap();

            for _ in 0..k {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let mut iter = input.split_whitespace();
                let go: usize = iter.next().unwrap().parse().unwrap();
                let cost: i32 = iter.next().unwrap().parse().unwrap();

                a.push(Date { go, back: u, cost });
                count += 1;
            }
        }

        root(&a, &mut d);

        for i in 0..n {
            println!("{} {}", i, d[i]);
        }
    }
