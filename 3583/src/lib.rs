use std::collections::HashMap;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        const MODULUS: usize = 1_000_000_007;
        let mut positions = HashMap::new();
        let mut res = 0;
        for (idx, n) in nums.iter().enumerate() {
            positions.entry(n).or_insert(vec![]).push(idx);
        }
        for (&&val, cnt) in &positions {
            if val == 0 {
                res += cnt.len() * (cnt.len() - 1) * (cnt.len() - 2) / 6;
                res %= MODULUS;
            } else {
                let double_val = val * 2;
                if let Some(half_positions) = positions.get(&double_val) {
                    for pos in cnt {
                        let partition_point =
                            half_positions.partition_point(|half_pos| half_pos < pos);
                        res += half_positions[..partition_point].len()
                            * half_positions[partition_point..].len();
                        res %= MODULUS;
                    }
                }
            }
        }
        res as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![6, 3, 6];
        let res = 1;
        assert_eq!(Solution::special_triplets(nums), res);
    }
}
