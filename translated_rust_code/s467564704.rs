use std::io;

    fn main() {
        let mut input = [[0; 4]; 4];
        let mut tc: i32;

        for i in 0..3 {
            for j in 0..3 {
                let mut input_str = String::new();
                io::stdin().read_line(&mut input_str).unwrap();
                input[i][j] = input_str.trim().parse().unwrap();
            }
        }

        let mut tc_str = String::new();
        io::stdin().read_line(&mut tc_str).unwrap();
        tc = tc_str.trim().parse().unwrap();

        for _ in 0..tc {
            let mut testcase_str = String::new();
            io::stdin().read_line(&mut testcase_str).unwrap();
            let testcase: i32 = testcase_str.trim().parse().unwrap();

            for i in 0..3 {
                for j in 0..3 {
                    if input[i][j] == testcase {
                        input[i][j] = 0;
                    }
                }
            }
        }

        if input[0][0] == 0 && input[1][1] == 0 && input[2][2] == 0 {
            println!("Yes");
        } else if input[0][2] == 0 && input[1][1] == 0 && input[2][0] == 0 {
            println!("Yes");
        } else if input[0][0] == 0 && input[0][1] == 0 && input[0][2] == 0 {
            println!("Yes");
        } else if input[1][0] == 0 && input[1][1] == 0 && input[1][2] == 0 {
            println!("Yes");
        } else if input[2][0] == 0 && input[2][1] == 0 && input[2][2] == 0 {
            println!("Yes");
        } else if input[0][0] == 0 && input[1][0] == 0 && input[2][0] == 0 {
            println!("Yes");
        } else if input[0][1] == 0 && input[1][1] == 0 && input[2][1] == 0 {
            println!("Yes");
        } else if input[0][2] == 0 && input[1][2] == 0 && input[2][2] == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
