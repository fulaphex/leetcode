impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let mut res = i32::MAX;

        for (x, y) in std::iter::zip(nums.iter(), nums.iter().skip(k as usize - 1)) {
            res = res.min(y - x);
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![90];
        let k = 1;
        let res = 0;
        assert_eq!(Solution::minimum_difference(nums, k), res);
    }

    #[test]
    fn test2() {
        let nums = vec![9, 4, 1, 7];
        let k = 2;
        let res = 2;
        assert_eq!(Solution::minimum_difference(nums, k), res);
    }
}
