impl Solution {
    fn is_no_zero(mut n: i32) -> bool {
        while n > 0 {
            if n % 10 == 0 {
                return false;
            }
            n /= 10;
        }
        return true;
    }
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for a in 1..n {
            let b = n - a;
            if Self::is_no_zero(a) && Self::is_no_zero(b) {
                return vec![a, b];
            }
        }
        panic!();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let res = vec![1, 1];
        assert_eq!(Solution::get_no_zero_integers(n), res);
    }

    #[test]
    fn test2() {
        let n = 11;
        let res = vec![2, 9];
        assert_eq!(Solution::get_no_zero_integers(n), res);
    }
}
