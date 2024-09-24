use std::io;

    fn main() {
        let mut input = [[0; 2]; 1024];
        let mut ans = [[0; 2]; 1024];
        let mut t = 0;

        loop {
            for i in 0..1024 {
                ans[i][0] = 0;
                ans[i][1] = 0;
                input[i][0] = 0;
                input[i][1] = 0;
            }

            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let mut iter = input_line.split_whitespace();
            let w: usize = iter.next().unwrap().parse().unwrap();
            let n: usize = iter.next().unwrap().parse().unwrap();

            if w == 0 {
                break;
            }

            for i in 0..n {
                let mut input_line = String::new();
                io::stdin().read_line(&mut input_line).unwrap();
                let mut iter = input_line.split(',');
                input[i][0] = iter.next().unwrap().trim().parse().unwrap();
                input[i][1] = iter.next().unwrap().trim().parse().unwrap();
            }

            for i in 0..n {
                for j in (input[i][1]..=1023).rev() {
                    if ans[j][0] < input[i][0] + ans[j - input[i][1]][0] {
                        ans[j][0] = input[i][0] + ans[j - input[i][1]][0];
                        ans[j][1] = input[i][1] + ans[j - input[i][1]][1];
                    }
                }
            }

            let mut j = 0;
            while ans[w][0] == ans[w - j - 1][0] {
                j += 1;
            }

            println!("Case {}:\n{}\n{}", t + 1, ans[w - j][0], ans[w - j][1]);
            t += 1;
        }
    }
