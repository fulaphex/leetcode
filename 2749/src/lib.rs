impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let mut res = 1;
        let (n1, n2) = (num1 as i64, num2 as i64);
        loop {
            let x = n1 - res * n2;
            if x.count_ones() <= res as u32 {
                return res as i32;
            }
            if x < res {
                return -1;
            }
            res += 1;
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (num1, num2) = (3, -2);
        let res = 3;
        assert_eq!(Solution::make_the_integer_zero(num1, num2), res);
    }
}
