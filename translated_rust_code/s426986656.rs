use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let a: i32 = iter.next().unwrap().trim().parse().unwrap();
        let b: i32 = iter.next().unwrap().trim().parse().unwrap();
        let mut flag = false;

        for i in 1..=3 {
            if (a * b * i) % 2 == 1 {
                flag = true;
                break;
            }
        }

        if flag {
            println!("Yes");
        } else {
            println!("No");
        }
    }
