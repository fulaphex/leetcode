struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let log = (n as f64).log2() / (3_f64).log2();
        return 3_i32.pow(log as u32) == n;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 27;
        assert!(Solution::is_power_of_three(n));
    }

    #[test]
    fn test4() {
        let n = 26;
        assert!(!Solution::is_power_of_three(n));
    }

    #[test]
    fn test2() {
        let n = 0;
        assert!(!Solution::is_power_of_three(n));
    }

    #[test]
    fn test3() {
        let n = -1;
        assert!(!Solution::is_power_of_three(n));
    }
}
