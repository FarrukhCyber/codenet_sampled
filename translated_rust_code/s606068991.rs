use std::io;

    fn white(n: usize, a: &[i32]) {
        for i in 0..n - 1 {
            print!("{} ", a[i]);
        }
        println!("{}", a[n - 1]);
    }

    fn insertionsort(n: usize, a: &mut [i32]) {
        for i in 1..n {
            let mut j = i;
            while j > 0 && a[j - 1] > a[j] {
                a.swap(j - 1, j);
                j -= 1;
            }
            white(i + 1, a);
        }
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = [0; 100];
        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            a[i] = input.trim().parse().unwrap();
        }

        insertionsort(n, &mut a);
    }
