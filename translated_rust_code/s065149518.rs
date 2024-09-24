use std::io;

    const MAX: usize = 21;

    fn init(n: &mut usize, fld: &mut [[u8; MAX]; MAX]) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut coords = input.split_whitespace().map(|s| s.parse::<usize>().unwrap());

        for _ in 0..*n {
            let (a, b) = (coords.next().unwrap(), coords.next().unwrap());
            fld[b][a] = 1;
        }
    }

    fn move_robot(m: usize, n: &mut usize, fld: &mut [[u8; MAX]; MAX]) {
        let mut input = String::new();
        for _ in 0..m {
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace();
            let dir = parts.next().unwrap().chars().next().unwrap();
            let a = parts.next().unwrap().parse::<usize>().unwrap();

            let (dx, dy) = match dir {
                'N' => (0, 1),
                'E' => (1, 0),
                'S' => (0, -1),
                'W' => (-1, 0),
                _ => panic!("Invalid direction"),
            };

            let (mut x, mut y) = (10, 10);
            for _ in 0..a {
                x = (x as isize + dx) as usize;
                y = (y as isize + dy) as usize;

                if fld[y][x] == 1 {
                    fld[y][x] = 0;
                    *n -= 1;
                }
            }
        }
    }

    fn main() {
        let mut input = String::new();
        loop {
            io::stdin().read_line(&mut input).unwrap();
            let n = input.trim().parse::<usize>().unwrap();
            if n == 0 {
                break;
            }

            let mut fld = [[0u8; MAX]; MAX];
            init(&mut n, &mut fld);

            io::stdin().read_line(&mut input).unwrap();
            let m = input.trim().parse::<usize>().unwrap();

            move_robot(m, &mut n, &mut fld);

            println!("{}", if n == 0 { "Yes" } else { "No" });
        }
    }
