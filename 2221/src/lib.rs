impl Solution {
    const MOD: i32 = 10;
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        for x in (1..nums.len()).rev() {
            for i in 0..x {
                nums[i] = (nums[i] + nums[i + 1]) % Self::MOD;
            }
        }
        return nums[0];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4, 5];
        let res = 8;
        assert_eq!(Solution::triangular_sum(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![5];
        let res = 5;
        assert_eq!(Solution::triangular_sum(nums), res);
    }
}
