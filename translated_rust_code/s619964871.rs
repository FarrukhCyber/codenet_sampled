use std::io;

    fn sort(a: &mut [i64], left: usize, middle: usize, right: usize) -> i64 {
        let mut l_arr = a[left..middle].to_vec();
        let mut r_arr = a[middle..right].to_vec();
        l_arr.push(i64::MAX);
        r_arr.push(i64::MAX);
        let (mut l, mut r) = (0, 0);
        let mut cnt = 0;
        for i in left..right {
            if l_arr[l] <= r_arr[r] {
                a[i] = l_arr[l];
                l += 1;
            } else {
                a[i] = r_arr[r];
                r += 1;
                cnt += (middle - left - l) as i64;
            }
        }
        cnt
    }

    fn merge(a: &mut [i64], left: usize, right: usize) -> i64 {
        if left + 1 < right {
            let middle = (left + right) / 2;
            let x1 = merge(a, left, middle);
            let x2 = merge(a, middle, right);
            let x3 = sort(a, left, middle, right);
            x1 + x2 + x3
        } else {
            0
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        let mut a = vec![0; n];
        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            a[i] = input.trim().parse().unwrap();
        }
        println!("{}", merge(&mut a, 0, n));
    }
