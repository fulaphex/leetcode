impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        if nums.len() == 4 {
            for i in 0..nums.len() {
                for j in i + 1..nums.len() {
                    if nums[i] == nums[j] {
                        return nums[i];
                    }
                }
            }
            unreachable!();
        }
        for i in 0..nums.len() - 2 {
            if nums[i] == nums[i + 1] {
                return nums[i];
            }
            if nums[i] == nums[i + 2] {
                return nums[i];
            }
            if nums[i + 2] == nums[i + 1] {
                return nums[i + 1];
            }
        }
        unreachable!();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 3];
        let res = 3;
        assert_eq!(Solution::repeated_n_times(nums), res);
    }
}
