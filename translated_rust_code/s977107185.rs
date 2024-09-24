use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut arr = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            arr.push(num);
        }

        let mut count = 0;
        for i in 0..n - 1 {
            for j in (i + 1..n).rev() {
                if arr[j] < arr[j - 1] {
                    arr.swap(j, j - 1);
                    count += 1;
                }
            }
        }

        for i in 0..n {
            print!("{}", arr[i]);
            if i < n - 1 {
                print!(" ");
            }
        }
        println!();
        println!("{}", count);
    }
