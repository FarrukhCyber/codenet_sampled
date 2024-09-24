use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut parts = input.split_whitespace();
        let str = parts.next().unwrap();
        let p = parts.next().unwrap();

        let tmp = str.to_string();
        let str = format!("{}{}", str, tmp);

        let slen = str.len();
        let plen = p.len();

        let mut flag = false;
        for i in 0..slen - plen {
            if &str[i..i + plen] == p {
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
