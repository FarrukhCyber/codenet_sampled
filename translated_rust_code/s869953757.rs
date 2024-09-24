use std::io;

    const M: usize = 100;
    const WHI: i32 = 0;
    const GRA: i32 = 1;
    const BLA: i32 = 2;

    fn enqueue(q: &mut Vec<usize>, tail: &mut usize, x: usize) {
        q[*tail] = x;
        *tail = (*tail + 1) % M;
    }

    fn dequeue(q: &Vec<usize>, head: &mut usize) -> usize {
        let x = q[*head];
        *head = (*head + 1) % M;
        x
    }

    fn bfs(t: &Vec<Vec<i32>>, color: &mut Vec<i32>, d: &mut Vec<i32>, n: usize, s: usize) {
        let mut q = vec![0; M];
        let mut head = 0;
        let mut tail = 0;

        for i in 0..n {
            color[i] = WHI;
            d[i] = 1000000;
        }

        color[s] = GRA;
        d[s] = 0;
        enqueue(&mut q, &mut tail, s);

        while head != tail {
            let u = dequeue(&q, &mut head);
            for v in 0..n {
                if t[u][v] == 1 && color[v] == WHI {
                    color[v] = GRA;
                    d[v] = d[u] + 1;
                    enqueue(&mut q, &mut tail, v);
                }
            }
            color[u] = BLA;
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut t = vec![vec![0; M]; M];
        let mut color = vec![0; M];
        let mut d = vec![0; M];

        for i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let u: usize = iter.next().unwrap().parse().unwrap() - 1;
            let k: usize = iter.next().unwrap().parse().unwrap();

            for _ in 0..k {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let v: usize = input.trim().parse().unwrap() - 1;
                t[u][v] = 1;
            }
        }

        bfs(&t, &mut color, &mut d, n, 0);

        for i in 0..n {
            if d[i] == 1000000 {
                d[i] = -1;
            }
            println!("{} {}", i + 1, d[i]);
        }
    }
