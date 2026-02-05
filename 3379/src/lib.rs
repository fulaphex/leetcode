impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        for i in 0..n {
            res[i] = nums[(((i as i32 + nums[i]) % n as i32 + n as i32) % n as i32) as usize];
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
        let nums = vec![3, -2, 1, 1];
        let res = vec![1, 1, 1, 3];
        assert_eq!(Solution::construct_transformed_array(nums), res);
    }
}
