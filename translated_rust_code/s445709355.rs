use std::io;

    fn input() -> i32 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    }

    fn calc(n: i32, f1: &mut [i32; 100], f2: &mut [i32; 100]) {
        let k = n % 10;
        let n = n / 10;
        f1[10 * k as usize + n as usize] += 1;
        f2[n as usize * 10 + k as usize] += 1;
    }

    fn main() {
        let mut f1 = [0; 100];
        let mut f2 = [0; 100];
        let n = input();
        for i in 1..=n {
            calc(i, &mut f1, &mut f2);
        }
        let mut ans = 0;
        for i in 0..100 {
            ans += f1[i] * f2[i];
        }
        println!("{}", ans);
    }
