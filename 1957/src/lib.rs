struct Solution {}
impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res: Vec<char> = Vec::from_iter(s.chars().take(2));
        for a in s.chars().skip(2) {
            if a == res[res.len() - 1] && a == res[res.len() - 2] {
                continue;
            } else {
                res.push(a);
            }
        }
        return String::from_iter(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("leeetcode");
        let res = String::from("leetcode");
        assert_eq!(Solution::make_fancy_string(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("aaabaaaa");
        let res = String::from("aabaa");
        assert_eq!(Solution::make_fancy_string(s), res);
    }

    #[test]
    fn test3() {
        let s = String::from("aab");
        let res = String::from("aab");
        assert_eq!(Solution::make_fancy_string(s), res);
    }
}
