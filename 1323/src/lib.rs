struct Solution {}

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let (mut x, mut idx) = (num, 0);
        let mut res_idx = u32::MAX;

        while x > 0 {
            if x % 10 == 6 {
                res_idx = idx;
            }
            x /= 10;
            idx += 1;
        }
        return if res_idx != u32::MAX {
            num + 10_i32.pow(res_idx) * 3
        } else {
            num
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num = 9669;
        let res = 9969;
        assert_eq!(Solution::maximum69_number(num), res);
    }

    #[test]
    fn test2() {
        let num = 9969;
        let res = 9999;
        assert_eq!(Solution::maximum69_number(num), res);
    }

    #[test]
    fn test3() {
        let num = 9999;
        let res = 9999;
        assert_eq!(Solution::maximum69_number(num), res);
    }
}
