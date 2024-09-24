use std::io;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.trim().parse().unwrap()).collect();

        let a = numbers[0];
        let b = numbers[1];

        let mawari = a + a + b + b;
        let menseki = a * b;

        println!("{} {}", menseki, mawari);
    }
    `
    The given C code calculates the perimeter and area of a rectangle. The user is prompted to enter the length and width of the rectangle. The Rust code does the same thing. It uses the `std::io` module to read user input, and then it calculates the perimeter and area of the rectangle. The results are then printed to the console.
