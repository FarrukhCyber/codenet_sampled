use std::io;

    fn push(stack: &mut Vec<i32>, value: i32) {
        stack.push(value);
    }

    fn pop(stack: &mut Vec<i32>) -> i32 {
        stack.pop().unwrap()
    }

    fn main() {
        let mut stack = Vec::new();
        let mut input = String::new();

        while io::stdin().read_line(&mut input).unwrap() != 0 {
            let trimmed_input = input.trim();
            match trimmed_input {
                "+" => {
                    let chonda = pop(&mut stack);
                    let yamagami = pop(&mut stack);
                    push(&mut stack, chonda + yamagami);
                }
                "-" => {
                    let yamagami = pop(&mut stack);
                    let chonda = pop(&mut stack);
                    push(&mut stack, chonda - yamagami);
                }
                "*" => {
                    let chonda = pop(&mut stack);
                    let yamagami = pop(&mut stack);
                    push(&mut stack, chonda * yamagami);
                }
                _ => {
                    push(&mut stack, trimmed_input.parse::<i32>().unwrap());
                }
            }
            input.clear();
        }

        println!("{}", pop(&mut stack));
    }
