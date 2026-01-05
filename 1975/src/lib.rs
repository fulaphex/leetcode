impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let (mut min, mut res, mut negative_count) = (i32::MAX, 0, 0);

        for row in &matrix {
            for &x in row {
                if x < 0 {
                    negative_count += 1;
                }
                min = min.min(x.abs());
                res += x.abs() as i64;
            }
        }

        if negative_count % 2 == 1 {
            res -= 2 * min as i64;
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
        let matrix = [[1, -1], [-1, 1]]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 4;

        assert_eq!(Solution::max_matrix_sum(matrix), res);
    }
}
