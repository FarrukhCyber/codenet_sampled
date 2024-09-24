use std::io;

    fn main() {
        let mut input = String::new();

        loop {
            let mut an = [0; 20];
            let mut a1 = [0; 6];
            let mut a2 = [0; 6];

            io::stdin().read_line(&mut input).expect("Failed to read line");
            let values: Vec<_> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

            an[0] = values[0];
            let l = values[1];

            if l == 0 {
                break;
            }

            let mut aj = an[0];
            let mut end = 0;

            for k in 1..21 {
                let mut temp = aj;
                for i in (0..l).rev() {
                    a1[i] = temp % 10;
                    a2[i] = temp % 10;
                    temp /= 10;
                }

                for _ in 1..l {
                    for j in 1..l {
                        if a1[j - 1] < a1[j] {
                            let tmp = a1[j - 1];
                            a1[j - 1] = a1[j];
                            a1[j] = tmp;
                        }
                        if a2[j - 1] > a2[j] {
                            let tmp = a2[j - 1];
                            a2[j - 1] = a2[j];
                            a2[j] = tmp;
                        }
                    }
                }

                let mut d = 1;
                let mut max = 0;
                let mut min = 0;

                for i in (0..l).rev() {
                    max += (a1[i] % 10) * d;
                    min += (a2[i] % 10) * d;
                    d *= 10;
                }

                let def = max - min;

                for i in 0..k {
                    if def == an[i] {
                        println!("{} {} {}", i, def, k - i);
                        end = 1;
                        break;
                    }
                }

                if end == 1 {
                    break;
                }

                an[k] = def;
                aj = def;
            }
        }
    }
