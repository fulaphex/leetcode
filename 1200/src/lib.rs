impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort_unstable();
        let mut min_diff = i32::MAX;
        let mut res = vec![];
        for w in arr.windows(2) {
            let diff = w[1] - w[0];
            if diff < min_diff {
                res = vec![];
                min_diff = diff;
            }
            if diff == min_diff {
                res.push(w.to_vec());
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
        let arr = vec![4, 2, 1, 3];
        let res = [[1, 2], [2, 3], [3, 4]]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        assert_eq!(Solution::minimum_abs_difference(arr), res);
    }
}
