use std::io;

    #[derive(Copy, Clone, PartialEq, PartialOrd)]
    struct Card {
        s: char,
        v: i32,
    }

    fn merge(a: &mut [Card], n: usize, l: usize, m: usize, r: usize) {
        let mut left = a[l..m].to_vec();
        let mut right = a[m..r].to_vec();
        left.push(Card { s: ' ', v: i32::MAX });
        right.push(Card { s: ' ', v: i32::MAX });
        let (mut i, mut j) = (0, 0);
        for k in l..r {
            if left[i] <= right[j] {
                a[k] = left[i];
                i += 1;
            } else {
                a[k] = right[j];
                j += 1;
            }
        }
    }

    fn merge_sort(a: &mut [Card], n: usize, l: usize, r: usize) {
        if l + 1 < r {
            let m = (l + r) / 2;
            merge_sort(a, n, l, m);
            merge_sort(a, n, m, r);
            merge(a, n, l, m, r);
        }
    }

    fn partition(a: &mut [Card], n: usize, p: usize, r: usize) -> usize {
        let x = a[r];
        let mut i = p;
        for j in p..r {
            if a[j] <= x {
                a.swap(i, j);
                i += 1;
            }
        }
        a.swap(i, r);
        i
    }

    fn quick_sort(a: &mut [Card], n: usize, p: usize, r: usize) {
        if p < r {
            let q = partition(a, n, p, r);
            quick_sort(a, n, p, q - 1);
            quick_sort(a, n, q + 1, r);
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        let mut a = vec![Card { s: ' ', v: 0 }; n];
        let mut b = vec![Card { s: ' ', v: 0 }; n];

        for i in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.split_whitespace().collect();
            let s = parts[0].chars().next().unwrap();
            let v: i32 = parts[1].parse().unwrap();
            a[i] = Card { s, v };
            b[i] = Card { s, v };
        }

        merge_sort(&mut a, n, 0, n);
        quick_sort(&mut b, n, 0, n - 1);

        let stable = a.iter().zip(b.iter()).all(|(x, y)| x.s == y.s);

        println!("{}", if stable { "Stable" } else { "Not stable" });
        for card in &b {
            println!("{} {}", card.s, card.v);
        }
    }
