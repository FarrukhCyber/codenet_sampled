use std::io;
    use std::cmp;

    #[derive(Copy, Clone)]
    struct Edge {
        s: i64,
        g: i64,
        c: i64,
    }

    struct Graph {
        vcnt: usize,
        ecnt: usize,
        e: Vec<Edge>,
        id: Vec<usize>,
    }

    fn esort(a: &Edge, b: &Edge) -> cmp::Ordering {
        if a.s < b.s {
            cmp::Ordering::Less
        } else if a.s > b.s {
            cmp::Ordering::Greater
        } else if a.g < b.g {
            cmp::Ordering::Less
        } else {
            cmp::Ordering::Greater
        }
    }

    fn readgraph() -> Graph {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i64 = input.trim().parse().unwrap();

        let mut g = Graph {
            vcnt: n as usize,
            ecnt: 0,
            e: Vec::new(),
            id: vec![0; n as usize + 1],
        };

        for i in 0..n - 1 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let x: i64 = iter.next().unwrap().parse().unwrap();
            let y: i64 = iter.next().unwrap().parse().unwrap();
            let c: i64 = iter.next().unwrap().parse().unwrap();

            g.e.push(Edge { s: x, g: y, c: c });
            g.e.push(Edge { s: y, g: x, c: c });
        }

        g.ecnt = g.e.len();
        g.e.sort_by(esort);

        let mut p = 0;
        for i in 0..g.vcnt {
            while p < g.ecnt && g.e[p].s < i as i64 {
                p += 1;
            }
            g.id[i] = p;
        }

        g
    }

    fn tyokkeidfs(s: i64, tyokkeitemp: &mut Vec<i64>, g: &Graph) {
        for i in g.id[s as usize]..g.id[s as usize + 1] {
            if tyokkeitemp[g.e[i].g as usize] == 0 {
                tyokkeitemp[g.e[i].g as usize] = tyokkeitemp[s as usize] + g.e[i].c;
                tyokkeidfs(g.e[i].g, tyokkeitemp, g);
            }
        }
    }

    fn tyokkei(g: &Graph) -> i64 {
        let mut tyokkeitemp = vec![0; g.vcnt + 10];
        tyokkeitemp[0] = 1;
        tyokkeidfs(0, &mut tyokkeitemp, g);

        let mut m = 0;
        let mut mi = 0;
        for i in 0..g.vcnt {
            if tyokkeitemp[i] > m {
                m = tyokkeitemp[i];
                mi = i;
            }
        }

        for i in 0..g.vcnt {
            tyokkeitemp[i] = 0;
        }
        tyokkeitemp[mi] = 1;
        tyokkeidfs(mi as i64, &mut tyokkeitemp, g);

        for i in 0..g.vcnt {
            m = cmp::max(m, tyokkeitemp[i]);
        }

        m - 1
    }

    fn main() {
        let g = readgraph();
        println!("{}", tyokkei(&g));
    }
    `
