use std::collections::HashMap;

impl Solution {
    fn combs(mut freqs: HashMap<i32, usize>) -> Vec<Vec<i32>> {
        if freqs.is_empty() {
            return vec![vec![]];
        }
        let el = *freqs.keys().next().unwrap();
        let count = freqs.remove(&el).unwrap();

        let mut res = vec![];
        for subset in Self::combs(freqs) {
            for c in 0..=count {
                let mut curr_set = subset.clone();
                curr_set.extend(std::iter::repeat_n(el, c));
                res.push(curr_set);
            }
        }
        res
    }
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut freqs = HashMap::new();
        for x in nums {
            *freqs.entry(x).or_insert(0) += 1;
        }
        Self::combs(freqs)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 2];
        let res = vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 2],
            vec![2],
            vec![2, 2],
        ];

        let mut out = Solution::subsets_with_dup(nums);
        for x in out.iter_mut() {
            x.sort_unstable();
        }
        out.sort_unstable();
        assert_eq!(out, res);
    }

    #[test]
    fn test_empty() {
        let nums = vec![];
        let res = vec![vec![]];
        let mut out = Solution::subsets_with_dup(nums);
        out.sort();
        assert_eq!(out, res);
    }
}
