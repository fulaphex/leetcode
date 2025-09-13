use std::collections::HashSet;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        let vovel_set: HashSet<char> = HashSet::from_iter("aeiouAEIOU".chars());
        let mut chars: Vec<_> = s.chars().collect();
        let vovel_idxs: Vec<_> = chars
            .iter()
            .enumerate()
            .filter(|&(_idx, c)| vovel_set.contains(c))
            .collect();
        // println!("{:?}", vovel_idxs);
        let mut vovels: Vec<_> = vovel_idxs.iter().map(|&(_idx, c)| *c).collect();
        let idxs: Vec<_> = vovel_idxs.iter().map(|&(idx, _c)| idx).collect();
        vovels.sort();

        // println!("{:?}", chars);
        for (idx, c) in std::iter::zip(idxs, vovels) {
            chars[idx] = c;
        }
        // println!("{:?}", chars);
        //
        return String::from_iter(chars);
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("lEetcOde");
        let res = String::from("lEOtcede");
        assert_eq!(Solution::sort_vowels(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("lYmpH");
        let res = String::from("lYmpH");
        assert_eq!(Solution::sort_vowels(s), res);
    }
}
