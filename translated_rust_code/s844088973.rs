use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        let x: i32 = iter.next().unwrap().parse().unwrap();

        for i in (x-k+1)..=(x+k-1) {
            print!("{}", i);
            if i < x+k-1 {
                print!(" ");
            }
        }
        println!();
    }
