use std::cmp::{min, max};
    use std::ptr;

    fn strrstr(string: &str, pattern: &str) -> Option<usize> {
        let mut offset = 0;
        let mut last_p = None;

        while let Some(search_result) = string[offset..].find(pattern) {
            last_p = Some(offset + search_result);
            offset += search_result + 1;
        }

        last_p
    }

    fn comb(n: usize, r: usize) -> usize {
        let mut result = 1;

        for i in 0..r {
            result *= n - i;
            result /= i + 1;
        }

        result
    }

    fn gcd(mut m: u64, mut n: u64) -> u64 {
        while m % n != 0 {
            let temp = n;
            n = m % n;
            m = temp;
        }

        n
    }

    fn main() {
        let pattern_base = "keyence";
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();

        let len = s.len();
        let mut found = false;

        for offset in 0..pattern_base.len() {
            let top = &pattern_base[0..offset];
            let bottom = &pattern_base[offset..];

            if let Some(top_search) = s.find(top) {
                if let Some(bottom_search) = strrstr(&s[top_search + offset..], bottom) {
                    let mut sep_count = 0;

                    if top_search != 0 {
                        sep_count += 1;
                    }
                    if top_search + offset != bottom_search {
                        sep_count += 1;
                    }
                    if bottom_search + bottom.len() != len {
                        sep_count += 1;
                    }

                    if sep_count <= 1 {
                        found = true;
                        break;
                    }
                }
            }
        }

        println!("{}", if found { "YES" } else { "NO" });
    }
