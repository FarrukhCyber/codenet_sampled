use std::io;

    fn get_int4() -> (i32, i32, i32, i32) {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let numbers: Vec<i32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        (numbers[0], numbers[1], numbers[2], numbers[3])
    }

    fn fget_array(size: usize) -> Vec<i64> {
        let mut arr = vec![0; size];
        for i in 0..size {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            arr[i] = input.trim().parse().unwrap();
        }
        arr
    }

    fn lower_bound(arr: &[i64], key: i64) -> usize {
        let mut low = 0;
        let mut high = arr.len();
        while low + 1 < high {
            let mid = (low + high) / 2;
            if arr[mid] < key {
                low = mid;
            } else {
                high = mid;
            }
        }
        low + 1
    }

    const NUM_MAX: usize = 1000;
    const RANK_MAX: usize = 3000;
    const VAL_MAX: i64 = 1e10 as i64;

    fn main() {
        let (x, y, z, rank) = get_int4();
        let mut xrr = fget_array(x as usize);
        let mut yrr = fget_array(y as usize);
        let mut zrr = fget_array(z as usize);
        xrr.sort();
        yrr.sort();
        zrr.sort();
        let min_sum = zrr.iter().min().unwrap() + xrr[0] + yrr[0];
        let max_sum = zrr.iter().max().unwrap() + xrr[x as usize - 1] + yrr[y as usize - 1];
        let mut pair = vec![0; x as usize * y as usize];
        let mut idx = 0;
        for i in 0..x {
            for j in 0..y {
                pair[idx] = xrr[i as usize] + yrr[j as usize];
                idx += 1;
            }
        }
        pair.sort();
        let min_sum = min_sum + pair[0];
        let max_sum = max_sum + pair[idx - 1];
        let mut cache = vec![0; z as usize];
        let mut cnt_map = vec![0; z as usize];
        let mut ans = vec![0; rank as usize];
        let size = x * y * z;
        let mut prev = min_sum;
        for r in (1..=rank).rev() {
            let mut low = prev - 1;
            let mut high = max_sum + 1;
            while low + 1 < high {
                let mid = (low + high) / 2;
                let mut cnt = 0;
                for i in 0..z {
                    let pivot = zrr[i as usize];
                    let tmp = lower_bound(&pair[cache[i as usize]..], mid - pivot);
                    cnt_map[i as usize] = tmp;
                    cnt += tmp;
                }
                if cnt <= size - r {
                    cache.copy_from_slice(&cnt_map);
                    low = mid;
                } else {
                    high = mid;
                }
            }
            ans[r as usize - 1] = low;
            if low == max_sum {
                break;
            }
            prev = low;
        }
        for i in 0..rank {
            println!("{}", if ans[i as usize] == 0 { max_sum } else { ans[i as usize] });
        }
    }
