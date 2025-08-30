impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let (mut max_val, mut max_sub, mut res) = (0, 0, 0);
        for num in nums {
            res = res.max(max_sub as i64 * num as i64);
            max_sub = max_sub.max(max_val - num);
            max_val = max_val.max(num);
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
        let nums = [12, 6, 1, 2, 7].into_iter().collect();
        let res = 77;
        assert_eq!(Solution::maximum_triplet_value(nums), res);
    }

    #[test]
    fn test2() {
        let nums = [1, 10, 3, 4, 19].into_iter().collect();
        let res = 133;
        assert_eq!(Solution::maximum_triplet_value(nums), res);
    }

    #[test]
    fn test3() {
        let nums = [1, 2, 3].into_iter().collect();
        let res = 0;
        assert_eq!(Solution::maximum_triplet_value(nums), res);
    }
}
