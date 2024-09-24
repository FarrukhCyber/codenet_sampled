use std::io;
    use std::collections::VecDeque;

    const MAX: usize = 100005;
    const N: usize = 100;

    #[derive(Clone)]
    struct P {
        name: [u8; N],
        t: i32,
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let q = iter.next().unwrap().parse::<i32>().unwrap();

        let mut queue = VecDeque::with_capacity(MAX);
        let mut t = 0;

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let name = iter.next().unwrap().as_bytes().to_vec();
            let t_val = iter.next().unwrap().parse::<i32>().unwrap();
            let mut name_arr = [0; N];
            name_arr[..name.len()].copy_from_slice(&name);
            queue.push_back(P { name: name_arr, t: t_val });
        }

        while !queue.is_empty() {
            let mut a = queue.pop_front().unwrap();
            if a.t > q {
                a.t -= q;
                t += q;
                queue.push_back(a);
            } else {
                t += a.t;
                let name = String::from_utf8_lossy(&a.name).trim_end_matches('\0').to_string();
                println!("{} {}", name, t);
            }
        }
    }
