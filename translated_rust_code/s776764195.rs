use std::io::{self, Read};

    const MAX_N: usize = 2000001;
    const MAX_Ai: usize = 10001;
    const STR_SIZE: usize = 10000010;

    fn get_num(input: &mut Vec<u8>) -> usize {
        let mut n = 0;
        while input[0] >= b'0' && input[0] <= b'9' {
            n = n * 10 + (input[0] - b'0') as usize;
            input.remove(0);
        }
        input.remove(0);
        n
    }

    fn put_num(n: usize, output: &mut Vec<u8>) {
        if n < 10 {
            output.push(b'0' + n as u8);
            return;
        }
        let d = n / 10;
        if d > 0 {
            put_num(d, output);
        }
        output.push(b'0' + (n - d * 10) as u8);
    }

    fn counting_sort(a: &mut [usize], b: &mut [usize], k: usize, len: usize) {
        let mut c = vec![0; k + 1];
        for j in 1..=len {
            c[a[j]] += 1;
        }
        for i in 1..=k {
            c[i] += c[i - 1];
        }
        for j in (1..=len).rev() {
            b[c[a[j]]] = a[j];
            c[a[j]] -= 1;
        }
    }

    fn main() {
        let mut input = vec![0; STR_SIZE];
        io::stdin().read_exact(&mut input).unwrap();

        let n = get_num(&mut input);
        let mut a = vec![0; MAX_N];
        for i in 1..=n {
            a[i] = get_num(&mut input);
        }

        let mut b = vec![0; MAX_N];
        counting_sort(&mut a, &mut b, MAX_Ai, n);

        let mut output = Vec::new();
        for i in 1..n {
            put_num(b[i], &mut output);
            output.push(b' ');
        }
        put_num(b[n], &mut output);
        output.push(b'\0');

        println!("{}", String::from_utf8(output).unwrap());
    }
