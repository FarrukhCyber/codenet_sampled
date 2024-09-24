use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();

        let mut a = vec![0; n];
        let mut tmpa = vec![0; n];
        let mut light = vec![0; n + 2];

        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            a[i] = input.trim().parse().unwrap();
        }

        for i in 0..n {
            let r = i as isize - a[i] as isize;
            let l = i as isize + a[i] as isize;
            light[if r < 0 { 0 } else { r as usize }] += 1;
            light[if l + 1 > n as isize { n + 1 } else { (l + 1) as usize }] -= 1;
        }

        for _ in 0..k {
            let mut sum = 0;
            for j in 0..n {
                tmpa[j] = a[j];
                sum += light[j];
                a[j] = sum;
            }
            for j in 0..n {
                let r = j as isize - tmpa[j] as isize;
                let l = j as isize + tmpa[j] as isize;
                light[if r < 0 { 0 } else { r as usize }] -= 1;
                light[if l + 1 > n as isize { n + 1 } else { (l + 1) as usize }] += 1;
                let r = j as isize - a[j] as isize;
                let l = j as isize + a[j] as isize;
                light[if r < 0 { 0 } else { r as usize }] += 1;
                light[if l + 1 > n as isize { n + 1 } else { (l + 1) as usize }] -= 1;
            }
            let mut is_same = true;
            for j in 0..n - 1 {
                if a[j] != a[j + 1] {
                    is_same = false;
                    break;
                }
            }
            if is_same && a[0] == n {
                break;
            }
        }

        for j in 0..n {
            print!("{}", a[j]);
            if j == n - 1 {
                println!();
            } else {
                print!(" ");
            }
        }
    }
