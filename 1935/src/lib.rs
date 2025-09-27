use std::collections::HashSet;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut res = 0;
        let broken_set: HashSet<char> = HashSet::from_iter(broken_letters.chars());
        for sub in text.split(" ") {
            let mut add = true;
            for l in sub.chars() {
                if broken_set.contains(&l) {
                    add = false;
                }
            }
            if add {
                res += 1;
            }
        }
        return res;
    }
}
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let text = String::from("hello world");
        let broken_letters = String::from("ad");
        let res = 1;
        assert_eq!(Solution::can_be_typed_words(text, broken_letters), res);
    }
}
