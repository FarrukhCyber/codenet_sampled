use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = Vec::new();
        let mut sum = 0;
        let mut wa = 0;

        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            a.push(num);
            sum += num;
            wa += i as i32 + 1;
        }

        let mut ans = if sum % wa == 0 { 1 } else { 0 };
        if ans == 1 {
            let shou = sum / wa;
            let mut sa = Vec::new();
            for i in 0..n {
                sa.push(a[(i + 1) % n] - a[i] - shou);
            }

            for i in 0..n - 1 {
                if sa[i] <= 0 && sa[i] % n as i32 == 0 {
                    ans = 1;
                } else {
                    ans = 0;
                    break;
                }
            }
        }

        if ans == 1 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
