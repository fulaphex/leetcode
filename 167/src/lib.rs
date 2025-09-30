impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0, numbers.len() - 1);
        loop {
            assert!(left < right);
            let sum = numbers[left] + numbers[right];
            if sum == target {
                break;
            } else if sum < target {
                left += 1;
            } else {
                right -= 1;
            }
        }
        return vec![left as i32 + 1, right as i32 + 1];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let numbers = vec![2, 7, 11, 15];
        let target = 9;
        let res = vec![1, 2];
        assert_eq!(Solution::two_sum(numbers, target), res);
    }

    #[test]
    fn test2() {
        let numbers = vec![2, 3, 4];
        let target = 6;
        let res = vec![1, 3];
        assert_eq!(Solution::two_sum(numbers, target), res);
    }

    #[test]
    fn test3() {
        let numbers = vec![-1, 0];
        let target = -1;
        let res = vec![1, 2];
        assert_eq!(Solution::two_sum(numbers, target), res);
    }
}
