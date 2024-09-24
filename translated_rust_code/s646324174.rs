use std::io;

    fn main() {
        let mut stack: Vec<i32> = Vec::new();

        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            match input.trim() {
                "+" => {
                    let b = stack.pop().expect("Stack underflow");
                    let a = stack.pop().expect("Stack underflow");
                    stack.push(a + b);
                }
                "-" => {
                    let b = stack.pop().expect("Stack underflow");
                    let a = stack.pop().expect("Stack underflow");
                    stack.push(a - b);
                }
                "*" => {
                    let b = stack.pop().expect("Stack underflow");
                    let a = stack.pop().expect("Stack underflow");
                    stack.push(a * b);
                }
                _ => {
                    let num = input.trim().parse::<i32>().expect("Invalid input");
                    stack.push(num);
                }
            }
        }

        println!("{}", stack.pop().expect("Stack underflow"));
    }
