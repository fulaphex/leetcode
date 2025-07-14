struct Solution {}
impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut iter = word.chars();
        let mut prev = iter.next().unwrap();
        let mut count = 1;
        let mut res = 1;
        for chr in iter {
            println!("char: {}, prev, count: {:?}", chr, (prev, count));
            if chr == prev {
                count += 1;
            } else {
                res += count - 1;
                prev = chr;
                count = 1;
            }
            println!("res: {}", res);
        }

        res += count - 1;
        return res;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test() {
        let word = String::from("abbcccc");
        let res = 5;
        assert_eq!(Solution::possible_string_count(word), res);
    }

    #[test]
    fn test2() {
        let word = String::from("abcd");
        let res = 1;
        assert_eq!(Solution::possible_string_count(word), res);
    }

    #[test]
    fn test3() {
        let word = String::from("aaaa");
        let res = 4;
        assert_eq!(Solution::possible_string_count(word), res);
    }
}
