use std::io;

    fn compare(a: &i32, b: &i32) -> std::cmp::Ordering {
        b.cmp(a)
    }

    fn main() {
        let mut array = [0; 10];
        for i in 0..10 {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            array[i] = input.trim().parse().expect("Please type a number!");
        }
        array.sort_by(compare);
        for i in 0..3 {
            println!("{}", array[i]);
        }
    }
