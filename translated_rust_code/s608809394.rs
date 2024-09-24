use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = Vec::new();
        for _ in 0..n {
            input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            a.push(num);
        }

        let mut t = 0;
        for i in 0..n {
            let mut s = 0;
            for j in (i+1)..n {
                let m = a[i] * a[j];
                s += m;
            }
            t += s;
        }

        println!("{}", t);
    }
