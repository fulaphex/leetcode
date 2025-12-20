impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        (0..strs[0].len())
            .map(|col| {
                if (1..strs.len())
                    .any(|idx| strs[idx - 1].bytes().nth(col) > strs[idx].bytes().nth(col))
                {
                    1
                } else {
                    0
                }
            })
            .sum()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = ["cba", "daf", "ghi"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let res = 1;

        assert_eq!(Solution::min_deletion_size(strs), res);
    }
}
