use std::io;

    struct Heap {
        data: Vec<i64>,
    }

    impl Heap {
        fn new() -> Heap {
            Heap { data: Vec::new() }
        }

        fn push(&mut self, x: i64) {
            let mut i = self.data.len();
            self.data.push(0);
            while i > 0 {
                let p = (i - 1) / 2;
                if self.data[p] >= x {
                    break;
                }
                self.data[i] = self.data[p];
                i = p;
            }
            self.data[i] = x;
        }

        fn pop(&mut self) -> Option<i64> {
            if self.data.is_empty() {
                return None;
            }
            let ret = self.data[0];
            let x = self.data.pop().unwrap();
            let mut i = 0;
            while i * 2 + 1 < self.data.len() {
                let a = i * 2 + 1;
                let b = i * 2 + 2;
                let a = if b < self.data.len() && self.data[b] > self.data[a] { b } else { a };
                if self.data[a] <= x {
                    break;
                }
                self.data[i] = self.data[a];
                i = a;
            }
            self.data[i] = x;
            Some(ret)
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();
        let mut a = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let num: i64 = input.trim().parse().unwrap();
            a.push(num);
        }
        let mut heap = Heap::new();
        for i in 0..n {
            heap.push(a[i]);
        }
        for _ in 0..m {
            let tm = heap.pop().unwrap();
            heap.push(tm / 2);
        }
        let mut ans = 0;
        for i in 0..n {
            ans += heap.data[i];
        }
        println!("{}", ans);
    }
