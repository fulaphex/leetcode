impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut vis = vec![false; 102];
        for (idx, &x) in nums.iter().enumerate().rev() {
            if vis[x as usize] {
                return (idx as i32) / 3 + 1;
            }
            vis[x as usize] = true;
        }
        return 0;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4, 2, 3, 3, 5, 7];
        let res = 2;
        assert_eq!(Solution::minimum_operations(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![4, 5, 6, 4, 4];
        let res = 2;
        assert_eq!(Solution::minimum_operations(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![6, 7, 8, 9];
        let res = 0;
        assert_eq!(Solution::minimum_operations(nums), res);
    }
}
