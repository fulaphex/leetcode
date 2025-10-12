use std::collections::HashMap;

#[inline]
fn comb(n: i128, k: i128) -> i128 {
    if k > n / 2 {
        return comb(n, n - k);
    }
    return (n - k + 1..=n).product::<i128>() / (1..=k).product::<i128>();
}

#[inline]
fn fastpow(x: i128, pow: i128, modulo: i128) -> i128 {
    if pow == 0 {
        return 1;
    }
    let xpowhalf = fastpow(x, pow / 2, modulo) % modulo;
    return (((xpowhalf * xpowhalf) % modulo) * if pow % 2 == 1 { x } else { 1 }) % modulo;
}

struct Solve {
    nums: Vec<i128>,
    sequence_len: i128,
    num_bits: i128,
    mem: HashMap<(i128, i128, usize, i128), i128>,
}

impl Solve {
    const MOD: i128 = 1_000_000_007;

    fn f(&mut self, remaining: i128, bits_remaining: i128, index: usize, carry: i128) -> i128 {
        if remaining < 0 || bits_remaining < 0 {
            return 0;
        }
        let carry_ones = carry.count_ones() as i128;
        if remaining + carry_ones < bits_remaining {
            return 0;
        }

        let key = (remaining, bits_remaining, index, carry);
        if self.mem.contains_key(&key) {
            return *self.mem.get(&key).unwrap();
        }
        if remaining == 0 {
            return if carry_ones == bits_remaining { 1 } else { 0 };
        }
        if index >= self.nums.len() {
            return 0;
        }

        let mut res = 0;
        for take in 0..=remaining {
            // let take_pow = (0..take).fold(1, |x, _| (x * self.nums[index]) % Self::MOD);
            let take_pow = fastpow(self.nums[index], take, Self::MOD);
            let perms = comb(remaining, take) % Self::MOD;

            let curr = carry + take;
            let x = self.f(
                remaining - take,
                bits_remaining - (curr % 2),
                index + 1,
                curr / 2,
            );
            res = (res + perms * take_pow * x) % Self::MOD;
        }
        self.mem.insert(key, res);

        return res;
    }
    fn solve(nums: Vec<i128>, m: i32, k: i32) -> i32 {
        return Solve {
            nums,
            sequence_len: m as i128,
            num_bits: k as i128,
            mem: HashMap::new(),
        }
        .f(m as i128, k as i128, 0, 0) as i32;
    }
}

impl Solution {
    pub fn magical_sum(m: i32, k: i32, nums: Vec<i32>) -> i32 {
        // m - length of sequence
        // k - number of bits
        let nums: Vec<i128> = nums.iter().map(|&x| x as i128).collect();

        return Solve::solve(nums, m, k);
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (m, k) = (5, 5);
        let nums = vec![1, 10, 100, 10000, 1000000];
        let res = 991600007;
        assert_eq!(Solution::magical_sum(m, k, nums), res);
    }

    #[test]
    fn test2() {
        let (m, k) = (2, 2);
        let nums = vec![1, 3];
        let res = 6;
        assert_eq!(Solution::magical_sum(m, k, nums), res);
    }

    #[test]
    fn test3() {
        let (m, k) = (3, 3);
        let nums = vec![52, 48];
        let res = 0;
        assert_eq!(Solution::magical_sum(m, k, nums), res);
    }

    #[test]
    fn test_big() {
        let (m, k) = (8, 8);
        let nums = vec![
            4475, 37658, 51018, 12424, 65157, 27914, 31161, 25310, 97672, 53516, 26018, 1860,
            47220, 27702, 77234, 6951, 22039, 9184, 64449, 45837, 58613, 53764, 24216, 73423,
            68676, 15003,
        ];
        let res = 369456900;
        assert_eq!(Solution::magical_sum(m, k, nums), res);
    }
}
