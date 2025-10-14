impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        let ndiv3 = n as u32 / 3;
        match n {
            2 => 1,
            3 => 2,
            _ => match n % 3 {
                //
                1 => 3_i32.pow(ndiv3 - 1) * 4,
                2 => 3_i32.pow(ndiv3) * 2,
                _ => 3_i32.pow(ndiv3),
            },
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let res = 1;
        assert_eq!(Solution::integer_break(n), res);
    }

    #[test]
    fn test2() {
        let n = 10;
        let res = 36;
        assert_eq!(Solution::integer_break(n), res);
    }

    #[test]
    fn test3() {
        let n = 3;
        let res = 2;
        assert_eq!(Solution::integer_break(n), res);
    }
}
