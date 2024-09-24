use std::io;

    const N: usize = 2000;

    fn main() {
        let mut math = [false; N + 1];
        math[0] = true;

        let mut num1 = String::new();
        io::stdin().read_line(&mut num1).unwrap();
        let num1: usize = num1.trim().parse().unwrap();

        let mut sum = 0;
        for _ in 0..num1 {
            let mut a = String::new();
            io::stdin().read_line(&mut a).unwrap();
            let a: usize = a.trim().parse().unwrap();

            sum += a;
            for j in (0..=sum - a).rev() {
                if math[j] {
                    math[j + a] = true;
                }
            }
        }

        let mut num2 = String::new();
        io::stdin().read_line(&mut num2).unwrap();
        let num2: usize = num2.trim().parse().unwrap();

        for _ in 0..num2 {
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).unwrap();
            let ans: usize = ans.trim().parse().unwrap();

            if math[ans] {
                println!("yes");
            } else {
                println!("no");
            }
        }
    }
