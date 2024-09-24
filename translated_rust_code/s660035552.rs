use std::io;

    fn part(a: &mut [i32], b: &mut [i32], l: usize, r: usize) -> usize {
        let mut i = l;
        let mut j = r;
        let x = b[r];

        loop {
            while b[i] < x {
                i += 1;
            }
            while i < j && x < b[j] {
                j -= 1;
            }

            if i >= j {
                break;
            }

            a.swap(i, j);
            b.swap(i, j);
        }

        a.swap(i, r);
        b.swap(i, r);
        i
    }

    fn quicksort(a: &mut [i32], b: &mut [i32], n: usize) {
        let mut low = vec![0];
        let mut high = vec![n - 1];

        while !low.is_empty() {
            let l = low.pop().unwrap();
            let r = high.pop().unwrap();

            if l < r {
                let v = part(a, b, l, r);

                if v - l < r - v {
                    low.push(v + 1);
                    high.push(r);
                    low.push(l);
                    high.push(v - 1);
                } else {
                    low.push(l);
                    high.push(v - 1);
                    low.push(v + 1);
                    high.push(r);
                }
            }
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let m: usize = iter.next().unwrap().parse().unwrap();

        let mut a = vec![0; m];
        let mut b = vec![0; m];

        for i in 0..m {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            a[i] = iter.next().unwrap().parse().unwrap();
            b[i] = iter.next().unwrap().parse().unwrap();
        }

        quicksort(&mut a, &mut b, m);

        let mut cnt = 0;
        let mut t = 0;
        for i in 0..m {
            if t <= a[i] {
                t = b[i];
                cnt += 1;
            }
        }

        println!("{}", cnt);
    }
