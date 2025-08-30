impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                for k in j + 1..nums.len() {
                    res = res.max((nums[i] - nums[j]) as i64 * nums[k] as i64);
                }
            }
        }
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![12, 6, 1, 2, 7];
        let res = 77;
        assert_eq!(Solution::maximum_triplet_value(nums), res);
    }
}
