use std::io;

    const MAX: usize = 100000;

    fn partition(c: &mut [char], a: &mut [i32], p: usize, r: usize) -> usize {
        let x = a[r];
        let y = c[r];
        let mut i = p;

        for j in p..r {
            if a[j] <= x {
                a.swap(i, j);
                c.swap(i, j);
                i += 1;
            }
        }

        a.swap(i, r);
        c.swap(i, r);

        i
    }

    fn quicksort(c: &mut [char], a: &mut [i32], p: usize, r: usize) {
        if p < r {
            let q = partition(c, a, p, r);
            quicksort(c, a, p, q);
            quicksort(c, a, q + 1, r);
        }
    }

    fn merge(mc: &mut [char], mn: &mut [i32], left: usize, mid: usize, right: usize) {
        let n1 = mid - left + 1;
        let n2 = right - mid;
        let mut ln = vec![0; n1 + 1];
        let mut rn = vec![0; n2 + 1];
        let mut lc = vec![' '; n1 + 1];
        let mut rc = vec![' '; n2 + 1];

        for i in 0..n1 {
            ln[i] = mn[left + i];
            lc[i] = mc[left + i];
        }

        for i in 0..n2 {
            rn[i] = mn[mid + i + 1];
            rc[i] = mc[mid + i + 1];
        }

        ln[n1] = i32::MAX;
        rn[n2] = i32::MAX;

        let (mut i, mut j) = (0, 0);

        for k in left..=right {
            if ln[i] <= rn[j] {
                mn[k] = ln[i];
                mc[k] = lc[i];
                i += 1;
            } else {
                mn[k] = rn[j];
                mc[k] = rc[j];
                j += 1;
            }
        }
    }

    fn merge_sort(mc: &mut [char], mn: &mut [i32], left: usize, right: usize) {
        if left < right {
            let mid = (left + right) / 2;
            merge_sort(mc, mn, left, mid);
            merge_sort(mc, mn, mid + 1, right);
            merge(mc, mn, left, mid, right);
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut c = vec![' '; MAX + 1];
        let mut a = vec![0; MAX + 1];
        let mut mc = vec![' '; MAX + 1];
        let mut mn = vec![0; MAX + 1];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            c[i] = iter.next().unwrap().chars().next().unwrap();
            a[i] = iter.next().unwrap().parse().unwrap();
            mc[i] = c[i];
            mn[i] = a[i];
        }

        merge_sort(&mut mc, &mut mn, 1, n);
        quicksort(&mut c, &mut a, 1, n);

        let mut count = 0;

        for i in 1..=n {
            if mn[i] == a[i] && mc[i] == c[i] {
                count += 1;
            }
        }

        println!("{}", if count == n { "Stable" } else { "Not stable" });

        for i in 1..=n {
            println!("{} {}", c[i], a[i]);
        }
    }
