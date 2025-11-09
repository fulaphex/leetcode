impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        let (low, hi) = (num1.min(num2), num1.max(num2));
        if low == 0 {
            return 0;
        }
        1 + Self::count_operations(low, hi - low)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (num1, num2) = (2, 3);
        let res = 3;
        assert_eq!(Solution::count_operations(num1, num2), res);
    }

    #[test]
    fn test2() {
        let (num1, num2) = (10, 10);
        let res = 1;
        assert_eq!(Solution::count_operations(num1, num2), res);
    }
}
