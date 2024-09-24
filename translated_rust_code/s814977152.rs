use std::io;

    fn miyukiti(asumisu: &i32, mizuhasu: &i32) -> std::cmp::Ordering {
        asumisu.cmp(mizuhasu)
    }

    fn sumipe(hayaminn: i32, tomatu: &Vec<i32>, yukinnko: usize) -> Option<usize> {
        let (mut satorina, mut zaasan) = (0, yukinnko - 1);
        while satorina <= zaasan {
            let yukati = satorina + (zaasan - satorina) / 2;
            if tomatu[yukati] == hayaminn {
                return Some(yukati);
            } else if tomatu[yukati] < hayaminn {
                satorina = yukati + 1;
            } else {
                zaasan = yukati - 1;
            }
        }
        None
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut a = Vec::new();
        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            a.push(num);
        }

        let mut tomatu = a.clone();
        tomatu.sort_by(miyukiti);
        tomatu.dedup();

        let yukinnko = tomatu.len();

        let mut mikakosi = vec![false; yukinnko];
        for &num in &a {
            if let Some(kugyu) = sumipe(num, &tomatu, yukinnko) {
                mikakosi[kugyu] = !mikakosi[kugyu];
            }
        }

        let emirinn = mikakosi.iter().filter(|&&x| x).count();
        println!("{}", emirinn);
    }
