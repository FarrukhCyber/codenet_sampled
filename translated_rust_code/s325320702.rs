use std::cmp::{max, min};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let l: i64 = iter.next().unwrap().parse().unwrap();
    let r: i64 = iter.next().unwrap().parse().unwrap();
    let d: i64 = iter.next().unwrap().parse().unwrap();

    let mut ans = 0;
    for i in l..=r {
        if i % d == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}


This Rust code does the same thing as the original C code. It reads three numbers from the standard input, `l`, `r`, and `d`. Then it counts the number of multiples of `d` in the range `l` to `r` (inclusive) and prints the result.
