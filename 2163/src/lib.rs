use std::{
    cmp::{self, Reverse},
    collections, iter,
};

struct Solution {}

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>) -> i64 {
        let n = nums.len() / 3;
        let mut pref_min: Vec<i64> = vec![1e12 as i64; n + 1];
        let mut suff_max: Vec<i64> = vec![0 as i64; n + 1];

        let mut heap = collections::BinaryHeap::new();
        let mut sum = 0 as i64;
        for &num in nums[0..n].iter() {
            sum += num as i64;
            heap.push(num as i64);
        }

        pref_min[0] = sum;
        for (idx, &num) in nums[n..2 * n].iter().enumerate() {
            sum += num as i64;
            heap.push(num as i64);
            sum -= heap.pop().unwrap();
            pref_min[idx + 1] = sum;
        }

        let mut min_heap = collections::BinaryHeap::<Reverse<i64>>::new();
        let mut sum = 0 as i64;
        for &num in nums.iter().rev().take(n) {
            sum += num as i64;
            min_heap.push(Reverse(num as i64));
        }
        suff_max[n] = sum;
        for (idx, &num) in nums[n..2 * n].iter().enumerate().rev() {
            sum += num as i64;
            min_heap.push(Reverse(num as i64));
            sum -= min_heap.pop().unwrap().0;
            suff_max[idx] = sum;
        }
        let mut res = 1e15 as i64;
        for (first, second) in iter::zip(pref_min, suff_max) {
            res = cmp::min(res, first - second);
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 1, 2];
        let res = -1;
        assert_eq!(Solution::minimum_difference(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![7, 9, 5, 8, 1, 3];
        let res = 1;
        assert_eq!(Solution::minimum_difference(nums), res);
    }
}
