use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let p = p as usize;
        let sum = nums.iter().fold(0, |acc, &x| (acc + x as usize) % p);
        if sum == 0 {
            return 0;
        }
        let (mut curr_sum, mut res) = (0, usize::MAX);
        let mut sums = HashMap::from([(0, 0)]);

        for (idx, &n) in nums.iter().enumerate() {
            let n = n as usize;
            curr_sum += n;
            curr_sum %= p;

            let start_sum = (curr_sum + p - sum) % p;

            let start_idx_opt = sums.get(&start_sum);
            if let Some(start_idx) = start_idx_opt {
                res = res.min(idx + 1 - start_idx);
            }

            sums.insert(curr_sum, idx + 1);
        }

        if res == nums.len() { -1 } else { res as i32 }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![3,1,4,2], 6, 1)]
    #[case(vec![6,3,5,2], 9, 2)]
    #[case(vec![1,2,3], 3, 0)]
    #[case(vec![5,12,2,14,11,15,10,22,20,10,20], 81, -1)]
    #[case(vec![8,32,31,18,34,20,21,13,1,27,23,22,11,15,30,4,2], 148, 7)]
    fn test(#[case] nums: Vec<i32>, #[case] p: i32, #[case] res: i32) {
        assert_eq!(Solution::min_subarray(nums, p), res);
    }
}
