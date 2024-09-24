use std::io;

    fn main() {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();
        let dimensions: Vec<_> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let hcell = dimensions[0];
        let wcell = dimensions[1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let paint: Vec<_> = input.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        let hpaint = paint[0];
        let wpaint = paint[1];

        let nblack = (wcell * hpaint) + (hcell * wpaint) - (hpaint * wpaint);
        let nwhite = (hcell * wcell) - nblack;

        println!("{}", nwhite);
    }
