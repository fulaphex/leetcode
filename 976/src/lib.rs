impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_by_key(|&x| -x);
        for x in nums.windows(3) {
            if x[0] < x[1] + x[2] {
                return x[0] + x[1] + x[2];
            }
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
        let nums = vec![2, 1, 2];
        let res = 5;
        assert_eq!(Solution::largest_perimeter(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 1, 10];
        let res = 0;
        assert_eq!(Solution::largest_perimeter(nums), res);
    }
}
