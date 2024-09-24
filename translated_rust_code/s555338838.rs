use std::io;

    fn main() {
        let mut len = String::new();
        io::stdin().read_line(&mut len).expect("Failed to read line");
        let len: usize = len.trim().parse().expect("Please type a number!");

        let mut a = Vec::with_capacity(len);
        for _ in 0..len {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let num: i32 = input.trim().parse().expect("Please type a number!");
            a.push(num);
        }

        for j in 1..len {
            let key = a[j];
            let mut i = j;
            while i > 0 && a[i - 1] > key {
                a[i] = a[i - 1];
                i -= 1;
            }
            a[i] = key;
        }

        for i in 0..len {
            if i != 0 {
                print!(" ");
            }
            print!("{}", a[i]);
        }
        println!();
    }
