use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, mut original: i32) -> i32 {
        let mut heap = BinaryHeap::from_iter(nums.iter().map(Reverse));
        while !heap.is_empty() {
            let x = *heap.pop().unwrap().0;
            if original == x {
                original *= 2;
            }
        }

        original
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![5, 6, 1, 3, 12];
        let original = 3;
        let res = 24;

        assert_eq!(Solution::find_final_value(nums, original), res);
    }
}
