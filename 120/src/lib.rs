impl Solution {
    const MAX: i32 = 10_000 * 500;
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let triangle_len = triangle.len();
        let mut dp_rows = vec![vec![Self::MAX; triangle_len + 1]; 2];
        for (idx, &f) in triangle[0].iter().enumerate() {
            dp_rows[0][idx + 1] = f;
        }
        for (row_idx, row) in triangle.into_iter().enumerate().skip(1) {
            for (idx, f) in row.into_iter().enumerate() {
                dp_rows[(row_idx) % 2][idx + 1] =
                    f + dp_rows[(row_idx - 1) % 2][idx].min(dp_rows[(row_idx - 1) % 2][idx + 1]);
            }
        }
        return *dp_rows[(triangle_len - 1) % 2].iter().min().unwrap();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let triangle = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let res = 11;
        assert_eq!(Solution::minimum_total(triangle), res);
    }

    #[test]
    fn test2() {
        let triangle = vec![vec![-10]];
        let res = -10;
        assert_eq!(Solution::minimum_total(triangle), res);
    }
}
