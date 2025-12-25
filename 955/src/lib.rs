use std::cmp::Ordering;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strlen = strs[0].len();
        let mut comparisons = vec![Ordering::Equal; strs.len() - 1];
        let mut res = 0;

        for col in 0..strlen {
            let mut new_comparisons = vec![];
            let mut update = true;

            for idx in 1..strs.len() {
                let (c1, c2) = (
                    strs[idx - 1].bytes().nth(col).unwrap(),
                    strs[idx].bytes().nth(col).unwrap(),
                );
                new_comparisons.push(c1.cmp(&c2));
                if comparisons[idx - 1] == Ordering::Equal && c1 > c2 {
                    res += 1;
                    update = false;
                    break;
                }
            }

            if update {
                for (old, new) in std::iter::zip(comparisons.iter_mut(), new_comparisons) {
                    if old == &Ordering::Equal && new == Ordering::Less {
                        *old = new;
                    }
                }
            }
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = ["ca", "bb", "ac"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let res = 1;

        assert_eq!(Solution::min_deletion_size(strs), res);
    }
}
