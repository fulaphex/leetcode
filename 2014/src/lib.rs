use std::collections::HashMap;

impl Solution {
    fn sequnces(
        seq: &[char],
        pattern_counts: &mut HashMap<char, i32>,
        pattern_counts_total: usize,
        acc: &mut Vec<char>,
        k: usize,
        result: &mut String,
    ) {
        if !Self::check_simple(seq, acc.as_slice(), k) {
            return;
        }
        if result.len() < acc.len() {
            *result = String::from_iter(acc.clone());
        }
        if acc.len() + pattern_counts_total <= result.len() {
            return;
        }

        let mut keys: Vec<char> = pattern_counts.keys().cloned().collect();
        keys.sort_by_key(|&x| u8::MAX - (x as u8));

        for key in keys {
            let val = pattern_counts.remove(&key).unwrap();
            if val > 1 {
                pattern_counts.insert(key, val - 1);
            }
            acc.push(key);
            Self::sequnces(
                seq,
                pattern_counts,
                pattern_counts_total - 1,
                acc,
                k,
                result,
            );
            acc.pop();
            pattern_counts.insert(key, val);
        }

        return;
    }

    fn check_simple(seq: &[char], pattern: &[char], k: usize) -> bool {
        if pattern.is_empty() {
            return true;
        }
        let mut pat = pattern;
        let mut left = pattern.len() * k as usize;

        for (idx, &c) in seq.iter().enumerate() {
            if (seq.len() - idx) < left {
                return false;
            }
            if c == pat[0] {
                pat = &pat[1..];
                left -= 1;
            }
            if pat.is_empty() {
                pat = pattern;
            }
            if left == 0 {
                return true;
            }
        }
        return false;
    }

    fn get_counts_slice(seq: &[char]) -> HashMap<char, i32> {
        let mut res = HashMap::new();
        for c in seq {
            *res.entry(*c).or_insert(0) += 1;
        }
        return res;
    }

    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let seq: Vec<char> = s.chars().collect();
        let seq_counts = Self::get_counts_slice(seq.as_slice());

        let mut pattern = vec![];
        for (&c, &v) in &seq_counts {
            for _ in 0..(v / k) {
                pattern.push(c);
            }
        }
        let mut pattern_counts = Self::get_counts_slice(pattern.as_slice());

        let mut res = String::new();
        Self::sequnces(
            seq.as_slice(),
            &mut pattern_counts,
            pattern.len(),
            &mut vec![],
            k as usize,
            &mut res,
        );
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("letsleetcode");
        let k = 2;
        let res = String::from("let");
        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), res);
    }

    #[test]
    fn test2() {
        let s = String::from(
            "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiijjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjjkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkklllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllllmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmmnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnnn",
        );
        let k = 251;
        let res = String::from("n");
        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), res);
    }

    #[test]
    fn test3() {
        let s = String::from("bwonderwonderwonderwonderwonderwonderwonderwonderb");
        let k = 8;
        let res = String::from("wonder");
        assert_eq!(Solution::longest_subsequence_repeated_k(s, k), res);
    }
}
