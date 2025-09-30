use std::collections::HashMap;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let mut subs = HashMap::new();
        for x in 0..s.len() - 9 {
            let sub = &s[x..][..10];
            *subs.entry(sub).or_insert(0) += 1;
        }
        subs.into_iter()
            .filter(|&(_, y)| y > 1)
            .map(|(x, _)| String::from(x))
            .collect::<Vec<_>>()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT");
        let res: Vec<String> = ["AAAAACCCCC", "CCCCCAAAAA"]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let mut out = Solution::find_repeated_dna_sequences(s);
        out.sort();
        assert_eq!(out, res);
    }

    #[test]
    fn test2() {
        let s = String::from("AAAAAAAAAAAAA");
        let res: Vec<String> = ["AAAAAAAAAA"]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let mut out = Solution::find_repeated_dna_sequences(s);
        out.sort();
        assert_eq!(out, res);
    }
}
