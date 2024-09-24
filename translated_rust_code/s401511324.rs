use std::io;

    fn main() {
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: usize = n.trim().parse().expect("Please type a number!");

        let mut l = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input: i32 = input.trim().parse().expect("Please type a number!");
            l.push(input);
        }

        l.sort();

        let mut cnt = 0;
        for i in 0..n-2 {
            for j in i+1..n-1 {
                for k in j+1..n {
                    if l[i] + l[j] > l[k] {
                        cnt += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        println!("{}", cnt);
    }
