impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut res = right & left;
        for x in 0..31 {
            let shift = 1 << x;
            let shifted = right - shift;
            if right & shift > 0 {
                if shifted >= left {
                    res &= shifted;
                } else {
                    break;
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
        let (left, right) = (5, 7);
        let res = 4;
        assert_eq!(Solution::range_bitwise_and(left, right), res);
    }

    #[test]
    fn test2() {
        let (left, right) = (0, 0);
        let res = 0;
        assert_eq!(Solution::range_bitwise_and(left, right), res);
    }

    #[test]
    fn test3() {
        let (left, right) = (1, 2147483647);
        let res = 0;
        assert_eq!(Solution::range_bitwise_and(left, right), res);
    }
}
