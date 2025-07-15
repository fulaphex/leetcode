use std::cmp;

struct Solution {}

impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        let mut s_iter = s.chars();
        let mut letter = s_iter.next().unwrap();
        let mut count = 1;
        let mut max1 = -1;
        let mut max0 = -1;
        for c in s_iter.chain(['2']) {
            if c == letter {
                count += 1;
            } else {
                if letter == '1' {
                    max1 = cmp::max(max1, count);
                } else {
                    max0 = cmp::max(max0, count);
                }
                letter = c;
                count = 1;
            }
        }
        println!("max0: {}; max1: {}", max0, max1);
        return max1 > max0;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("11001");
        assert!(!Solution::check_zero_ones(s));
    }

    #[test]
    fn test2() {
        let s = String::from("1101");
        assert!(Solution::check_zero_ones(s));
    }

    #[test]
    fn test3() {
        let s = String::from("111000");
        assert!(!Solution::check_zero_ones(s));
    }

    #[test]
    fn test4() {
        let s = String::from("110100010");
        assert!(!Solution::check_zero_ones(s));
    }
}
