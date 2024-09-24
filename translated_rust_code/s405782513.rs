use std::io;

    fn main() {
        let mut input = String::new();
        loop {
            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(n) if n > 0 => {
                    let mut c = [0; 3];
                    let mut card = [0; 10];
                    let mut count = 0;
                    let mut sum = 0;

                    let numbers: Vec<_> = input.split_whitespace().map(|s| s.trim().parse::<i32>().unwrap()).collect();
                    for (i, &num) in numbers.iter().enumerate() {
                        c[i] = num;
                        sum += num;
                        card[(num - 1) as usize] = 1;
                    }

                    for i in 0..10 {
                        if card[i] == 0 && sum + i as i32 + 1 <= 20 {
                            count += 1;
                        }
                    }

                    if count >= 4 {
                        println!("YES");
                    } else {
                        println!("NO");
                    }
                }
                _ => break,
            }
        }
    }
