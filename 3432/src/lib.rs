impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        if nums.iter().sum::<i32>() % 2 == 0 {
            nums.len() as i32 - 1
        } else {
            0
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![10, 10, 3, 7, 6];
        let res = 4;
        assert_eq!(Solution::count_partitions(nums), res);
    }

    #[test]
    fn test_none() {
        let nums = vec![10, 10, 4, 7, 6];
        let res = 0;
        assert_eq!(Solution::count_partitions(nums), res);
    }
}
