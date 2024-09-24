use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().unwrap()).collect();

        let a = numbers[0];
        let b = numbers[1];

        let x = 2 * a + 2 * b;
        let y = a * b;

        println!("{} {}", y, x);
    }
