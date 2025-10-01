impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let (mut res, mut sum, mut start) = (i32::MAX, 0, 0);
        for (end, el) in nums.iter().enumerate() {
            sum += el;
            while (sum - nums[start]) >= target {
                sum -= nums[start];
                start += 1;
            }
            if sum >= target {
                res = res.min((end - start + 1) as i32);
            }
        }
        if res == i32::MAX {
            return 0;
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
        let nums = vec![2, 3, 1, 2, 4, 3];
        let target = 7;
        let res = 2;
        assert_eq!(Solution::min_sub_array_len(target, nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 4, 4];
        let target = 4;
        let res = 1;
        assert_eq!(Solution::min_sub_array_len(target, nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let target = 11;
        let res = 0;
        assert_eq!(Solution::min_sub_array_len(target, nums), res);
    }
}
