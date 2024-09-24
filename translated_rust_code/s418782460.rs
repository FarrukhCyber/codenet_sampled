use std::io;

    const MAX_STRING: usize = 30;
    const MAX_LINE: usize = 2000000;
    const MAX_HUMAN: usize = 200000;

    fn main() {
        let mut s_input = String::new();
        let mut s_long_input_line = String::new();
        let mut an_coming_sec = [0; MAX_HUMAN];

        let mut n_human_number = 0;
        let mut n_next_human_index = 0;
        let mut n_water_length = 0;
        let mut n_water_life = 0;
        let mut n_start = 0;
        let mut n_stop = 0;

        io::stdin().read_line(&mut s_input).expect("Failed to read line");
        let mut split = s_input.split_whitespace();
        n_human_number = split.next().unwrap().parse().unwrap();
        n_water_length = split.next().unwrap().parse().unwrap();

        io::stdin().read_line(&mut s_long_input_line).expect("Failed to read line");
        let mut split = s_long_input_line.split_whitespace();
        for i in 0..n_human_number {
            an_coming_sec[i] = split.next().unwrap().parse().unwrap();
        }

        n_start = an_coming_sec[0];

        while n_next_human_index < n_human_number {
            if n_next_human_index == n_human_number {
                break;
            } else {
                n_stop = n_start + n_water_length;
                if an_coming_sec[n_next_human_index] < n_stop {
                    while n_next_human_index < n_human_number && an_coming_sec[n_next_human_index] < n_stop {
                        if n_stop <= an_coming_sec[n_next_human_index] + n_water_length {
                            n_stop = an_coming_sec[n_next_human_index] + n_water_length;
                            n_next_human_index += 1;
                        }
                    }
                    if n_next_human_index == n_human_number {
                        n_water_life += n_stop - n_start;
                        break;
                    }
                }
                n_water_life += n_stop - n_start;
                n_start = n_stop;
                if n_start < an_coming_sec[n_next_human_index] {
                    n_start = an_coming_sec[n_next_human_index];
                }
            }
        }
        println!("{}", n_water_life);
    }
