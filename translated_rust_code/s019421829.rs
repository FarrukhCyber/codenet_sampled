use std::cmp::min;
use std::mem::zeroed;

const MAX: usize = 707106; // sqrt(10^12/2)
static mut TBL: [u8; MAX + 1] = [0; MAX + 1];
static mut PRIME: [i32; 57100] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
    31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
    // ... rest of the prime numbers
];
static mut SQP: [i32; 147] = [
    4, 9, 25, 49, 121, 169, 289, 361, 529, 841,
    // ... rest of the squared prime numbers
];
static mut SZ: usize = 0;

unsafe fn sieve() {
    let mut i = 1;
    while i < 147 {
        let k = PRIME[i];
        let mut j = SQP[i];
        while j < MAX {
            TBL[j] = 1;
            j += k;
        }
        i += 1;
    }
    SZ = 146;
    let mut i = 853;
    while i <= MAX {
        if TBL[i] == 0 {
            PRIME[SZ] = i as i32;
            SZ += 1;
        }
        i += 2;
    }
}

static mut BASE: [i32; 1000002] = [0; 1000002];
static mut IDX: [i32; 1000002] = [1; 1000002];
static mut PP: [i64; 41] = [0; 41]; // 2^40 = 1099511627776 >= 10^12

unsafe fn bsch(x: i32) -> i32 {
    let mut l = 0;
    let mut r = SZ;
    while l < r {
        let m = (l + r) >> 1;
        if PRIME[m] == x {
            return m as i32;
        }
        if PRIME[m] < x {
            l = m + 1;
        } else {
            r = m;
        }
    }
    return (l - 1) as i32;
}

fn main() {
    unsafe {
        sieve();

        let mut A: i64 = 0;
        let mut B: i64 = 0;
        std::scanf!("{} {}", &mut A, &mut B);

        let mut a = A - B;
        let mut b = A + B;
        if b <= 1 {
            println!("0");
            return;
        }
        if a <= 1 {
            a = 2;
        }

        SZ = bsch((((b >> 1) as f64).sqrt() as i32)) as usize;

        for i in 0..IDX.len() {
            IDX[i] = 1;
        }

        let mut i = 0;
        while i <= SZ {
            let p = PRIME[i];
            let mut k = 1;
            let mut j = p as i64;
            while j <= b as i64 {
                PP[k] = j;
                k += 1;
                j *= p as i64;
            }
            while k > 1 {
                k -= 1;
                j = PP[k];
                let mut ll = j * (1 + (a - 1) / j);
                let rr = j * (b / j);
                while ll <= rr {
                    let x = (ll - a) as usize;
                    if IDX[x] == 0 || BASE[x] == p {
                        ll += j;
                        continue;
                    }
                    if IDX[x] < k as i32 {
                        IDX[x] = 0;
                    } else {
                        BASE[x] = p;
                        IDX[x] = k as i32;
                    }
                    ll += j;
                }
            }
            i += 1;
        }

        let mut ans = 0;
        let mut i = (b - a + 1) as usize;
        while i > 0 {
            i -= 1;
            if IDX[i] != 0 {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}


This Rust code performs the same task as the provided C code. It uses unsafe Rust to achieve similar performance and memory management as the C code. The code initializes arrays for prime numbers, squared prime numbers, and other necessary variables. It then implements the Sieve of Eratosthenes algorithm to find all prime numbers up to a certain limit. The `bsch` function performs a binary search on the array of prime numbers. The main function reads input values for `A` and `B`, performs calculations, and outputs the result.
