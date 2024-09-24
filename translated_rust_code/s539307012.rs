use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = iter.next().unwrap();
        let mut x = iter.next().unwrap();
        let mut min = iter.next().unwrap();
        x -= min;
        for _ in 1..n {
            let m = iter.next().unwrap();
            x -= m;
            if min > m {
                min = m;
            }
        }
        println!("{}", n + x / min);
    }
