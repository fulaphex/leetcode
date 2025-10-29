impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        2_i32.pow(n.ilog2() + 1).max(2) - 1
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 5;
        let res = 7;
        assert_eq!(Solution::smallest_number(n), res);
    }

    #[test]
    fn test2() {
        let n = 10;
        let res = 15;
        assert_eq!(Solution::smallest_number(n), res);
    }

    #[test]
    fn test3() {
        let n = 3;
        let res = 3;
        assert_eq!(Solution::smallest_number(n), res);
    }
}
