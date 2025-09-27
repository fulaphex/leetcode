struct Solution {}

impl Solution {
    pub fn largest_good_integer(num: String) -> String {
        let (mut prev, mut count, mut res) = (0 as char, 0, 0 as char);
        for chr in num.chars() {
            if chr == prev {
                count += 1;
            } else {
                (prev, count) = (chr, 1);
            }
            println!("{} {}", prev, count);
            if count >= 3 && chr > res {
                res = chr;
            }
        }
        if res == (0 as char) {
            return String::from("");
        }
        return String::from_iter([res; 3]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num = String::from("6777133339");
        assert_eq!(Solution::largest_good_integer(num), "777");
    }

    #[test]
    fn test2() {
        let num = String::from("2300019");
        assert_eq!(Solution::largest_good_integer(num), "000");
    }

    #[test]
    fn test3() {
        let num = String::from("42352338");
        assert_eq!(Solution::largest_good_integer(num), "");
    }
}
