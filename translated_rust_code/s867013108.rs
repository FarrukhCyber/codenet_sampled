use std::io;

    const DAYS: usize = 365;

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut nyuyoku_ticket = [0; DAYS];
        let mut pool_ticket = [0; DAYS];
        let mut nyuyoku_member = [0; DAYS];
        let mut pool_member = [0; DAYS];
        let mut saiyasune = [0; DAYS];

        for i in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let values: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
            nyuyoku_ticket[i] = values[0];
            pool_ticket[i] = values[1];
            nyuyoku_member[i] = values[2];
            pool_member[i] = values[3];

            let seiki_ryoukin = nyuyoku_ticket[i] * nyuyoku_member[i] + pool_ticket[i] * pool_member[i];
            let mut waribiki_ryoukin = 0;

            if nyuyoku_member[i] >= 5 && pool_member[i] >= 2 {
                waribiki_ryoukin = (seiki_ryoukin * 8) / 10;
            } else if nyuyoku_member[i] >= 5 {
                waribiki_ryoukin = (nyuyoku_ticket[i] * nyuyoku_member[i] + pool_ticket[i] * 2) * 8 / 10;
            } else if pool_member[i] >= 2 {
                waribiki_ryoukin = (nyuyoku_ticket[i] * 5 + pool_ticket[i] * pool_member[i]) * 8 / 10;
            } else {
                waribiki_ryoukin = (nyuyoku_ticket[i] * 5 + pool_ticket[i] * 2) * 8 / 10;
            }

            saiyasune[i] = seiki_ryoukin.min(waribiki_ryoukin);
        }

        for i in 0..n {
            println!("{}", saiyasune[i]);
        }
    }
