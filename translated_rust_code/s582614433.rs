use std::cmp::Ordering;
    use std::slice::sort_by;

    #[derive(Eq)]
    struct Edge {
        p: i64,
        i: i64,
        s: String,
    }

    impl PartialEq for Edge {
        fn eq(&self, other: &Self) -> bool {
            self.s == other.s && self.p == other.p
        }
    }

    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            match self.s.cmp(&other.s) {
                Ordering::Equal => Some(other.p.cmp(&self.p)),
                ord => Some(ord),
            }
        }
    }

    impl Ord for Edge {
        fn cmp(&self, other: &Self) -> Ordering {
            self.partial_cmp(other).unwrap()
        }
    }

    fn main() {
        let mut data: Vec<Edge> = Vec::new();
        let n = 5; // replace with input

        for i in 0..n {
            let s = String::from("example"); // replace with input
            let p = 10; // replace with input
            data.push(Edge { p, i, s });
        }

        data.sort();

        for i in 0..n {
            println!("{}", data[i as usize].i + 1);
        }
    }
