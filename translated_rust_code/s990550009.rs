use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: usize = n.trim().parse().expect("Please type a number!");

        let mut a = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = input.trim().parse().expect("Please type a number!");
            a.push(input);
        }

        let mut cnt = 1;
        let mut isup = a[0] < a[1];
        let mut issame = a[0] == a[1];

        for i in 1..n - 1 {
            if issame {
                if a[i] != a[i + 1] {
                    issame = false;
                    isup = a[i] < a[i + 1];
                }
            } else {
                if (isup && a[i] <= a[i + 1]) || (!isup && a[i] >= a[i + 1]) {
                    continue;
                } else {
                    if i + 2 < n {
                        if a[i + 1] < a[i + 2] {
                            isup = true;
                        } else if a[i + 1] > a[i + 2] {
                            isup = false;
                        } else {
                            issame = true;
                        }
                        cnt += 1;
                    } else {
                        cnt += 1;
                        break;
                    }
                }
            }
        }

        println!("{}", cnt);
    }
