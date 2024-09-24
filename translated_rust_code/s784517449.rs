use std::io;

    fn binary_search(arr: &[i32], key: i32) -> bool {
        let mut left = 0;
        let mut right = arr.len();

        while left < right {
            let mid = (left + right) / 2;
            if key == arr[mid] {
                return true;
            }
            if key > arr[mid] {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        false
    }

    fn main() {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        let mut arr = Vec::new();
        for _ in 0..n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let num: i32 = input.trim().parse().unwrap();
            arr.push(num);
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let q: usize = input.trim().parse().unwrap();

        let mut sum = 0;
        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let k: i32 = input.trim().parse().unwrap();
            if binary_search(&arr, k) {
                sum += 1;
            }
        }

        println!("{}", sum);
    }
