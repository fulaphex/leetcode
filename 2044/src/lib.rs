struct Solution {}

impl Solution {
    pub fn _inner(nums: &[i32], target: i32, acc: i32) -> i32 {
        if target == acc {
            return 1 << nums.len();
        }
        if nums.len() == 0 {
            return 0;
        }

        // taking the first element
        return Self::_inner(&nums[1..], target, acc | nums[0]) 
        // leaving the first element out
            + Self::_inner(&nums[1..], target, acc);
    }

    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        println!("{:?}", nums);
        let target: i32 = nums.iter().map(|&x| x).reduce(|x, y| x | y).unwrap();
        println!("target: {}", target);

        let res = Self::_inner(&nums, target, 0);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 1];
        let res = 2;
        assert_eq!(Solution::count_max_or_subsets(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![2, 2, 2];
        let res = 7;
        assert_eq!(Solution::count_max_or_subsets(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![3, 2, 1, 5];
        let res = 6;
        assert_eq!(Solution::count_max_or_subsets(nums), res);
    }

    #[test]
    fn test4() {
        let nums = Vec::from_iter(1..16);
        let res = 32297;
        assert_eq!(Solution::count_max_or_subsets(nums), res);
    }
}
