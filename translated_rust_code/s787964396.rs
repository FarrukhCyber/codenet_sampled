use std::io;

    const I: usize = 10000;

    fn counting_sort(a: &mut [usize], b: &mut [usize], k: usize) {
        let mut c = vec![0; I + 1];
        for j in 0..k {
            c[a[j + 1]] += 1;
        }
        for i in 1..=I {
            c[i] += c[i - 1];
        }
        for j in (0..k).rev() {
            b[c[a[j + 1]] - 1] = a[j + 1];
            c[a[j + 1]] -= 1;
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = vec![0; n + 1];
        let mut b = vec![0; n + 1];

        for i in 0..n {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            a[i + 1] = input.trim().parse().unwrap();
        }

        counting_sort(&mut a, &mut b, n);

        print!("{}", b[0]);
        for i in 1..n {
            print!(" {}", b[i]);
        }
        println!();
    }
