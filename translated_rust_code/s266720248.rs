use std::io;
    use std::collections::VecDeque;

    const LEN: usize = 100005;

    #[derive(Clone)]
    struct P {
        name: String,
        t: i32,
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let q = iter.next().unwrap().parse::<i32>().unwrap();

        let mut queue = VecDeque::with_capacity(LEN);
        for _ in 0..n {
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            name = name.trim().to_string();

            let mut t = String::new();
            io::stdin().read_line(&mut t).unwrap();
            let t = t.trim().parse::<i32>().unwrap();

            queue.push_back(P { name, t });
        }

        let mut elaps = 0;
        while !queue.is_empty() {
            let mut u = queue.pop_front().unwrap();
            let c = i32::min(q, u.t);
            u.t -= c;
            elaps += c;
            if u.t > 0 {
                queue.push_back(u);
            } else {
                println!("{} {}", u.name, elaps);
            }
        }
    }
