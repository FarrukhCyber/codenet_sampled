use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: i64 = iter.next().unwrap().parse().unwrap();
        let k: i64 = iter.next().unwrap().parse().unwrap();
        let q: i64 = iter.next().unwrap().parse().unwrap();

        let mut arr = vec![0; n as usize];

        for _ in 0..q {
            let mut temp = String::new();
            io::stdin().read_line(&mut temp).unwrap();
            let temp: usize = temp.trim().parse().unwrap();
            arr[temp - 1] += 1;
        }

        for i in 0..n as usize {
            if arr[i] > (q - k) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
