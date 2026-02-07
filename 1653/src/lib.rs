impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut curr = s.bytes().filter(|&x| x == b'a').count();
        let mut res = curr;
        for c in s.bytes() {
            if c == b'a' {
                curr -= 1;
            } else {
                curr += 1;
            }
            res = res.min(curr);
        }
        res as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("aababbab");
        let res = 2;
        assert_eq!(Solution::minimum_deletions(s), res);
    }
}
