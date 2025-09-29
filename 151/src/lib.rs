impl Solution {
    pub fn reverse_words(s: String) -> String {
        let parts: Vec<_> = s.split(" ").filter(|&x| x.len() > 0).collect();
        return parts.into_iter().rev().collect::<Vec<_>>().join(" ");
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("the sky is blue");
        let res = String::from("blue is sky the");
        assert_eq!(Solution::reverse_words(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("  hello world  ");
        let res = String::from("world hello");
        assert_eq!(Solution::reverse_words(s), res);
    }
}
