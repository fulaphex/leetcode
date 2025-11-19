use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn nth_super_ugly_number(n: i32, primes: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut res = vec![1];
        res.reserve(n);
        let mut multiplies_idxs = vec![0; primes.len()];

        let mut heap =
            BinaryHeap::from_iter(primes.iter().enumerate().map(|(idx, &p)| (Reverse(p), idx)));

        while res.len() < n {
            let (Reverse(min_val), min_idx) = heap.pop().unwrap();
            if min_val != res[res.len() - 1] {
                res.push(min_val);
            }

            multiplies_idxs[min_idx] += 1;
            if let Some(new_val) = primes[min_idx].checked_mul(res[multiplies_idxs[min_idx]]) {
                heap.push((Reverse(new_val), min_idx));
            }
        }
        res[res.len() - 1]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 12;
        let primes = vec![2, 7, 13, 19];
        let res = 32;

        assert_eq!(Solution::nth_super_ugly_number(n, primes), res);
    }

    #[test]
    fn test2() {
        let n = 10_000;
        let primes = vec![
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541,
        ];
        let res = 16125;

        assert_eq!(Solution::nth_super_ugly_number(n, primes), res);
    }

    #[test]
    fn test3() {
        let n = 100_000;
        let primes = vec![
            7, 19, 29, 37, 41, 47, 53, 59, 61, 79, 83, 89, 101, 103, 109, 127, 131, 137, 139, 157,
            167, 179, 181, 199, 211, 229, 233, 239, 241, 251,
        ];
        let res = 1092889481;

        assert_eq!(Solution::nth_super_ugly_number(n, primes), res);
    }

    #[test]
    fn test4() {
        let n = 5911;
        let primes = vec![2, 3, 5, 7];
        let res = 2144153025;

        assert_eq!(Solution::nth_super_ugly_number(n, primes), res);
    }
}
