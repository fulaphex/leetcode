impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut digits: Vec<u8> = s.as_bytes().iter().map(|&x| x - b'0').collect();

        for iter in (2..digits.len()).rev() {
            for idx in 0..iter {
                digits[idx] = (digits[idx] + digits[idx + 1]) % 10;
            }
        }

        digits[0] == digits[1]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("3902");
        let res = true;
        assert_eq!(Solution::has_same_digits(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("34789");
        let res = false;
        assert_eq!(Solution::has_same_digits(s), res);
    }
}
