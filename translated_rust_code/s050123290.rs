use std::io;

    fn main() {
        let mut su = [0u8; 1000];
        let mut susum = 0;
        let mut j = 0;

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            for c in input.bytes() {
                su[j] = c;
                if su[j] == b'\n' {
                    break;
                }
                susum += (su[j] - b'0') as i32;
                j += 1;
            }

            if su[0] == b'0' {
                break;
            }

            println!("{}", susum);
            susum = 0;
            j = 0;
        }
    }
