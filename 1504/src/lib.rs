struct Solution {}

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let (row_count, col_count) = (mat.len(), mat[0].len());
        let mut earliest_continuous = vec![vec![-1; col_count]; row_count];

        for row_idx in 0..row_count {
            let mut earliest = -1;
            for col_idx in 0..col_count {
                let field = mat[row_idx][col_idx];
                if field == 1 {
                    if earliest == -1 {
                        earliest = col_idx as i32;
                    }
                    earliest_continuous[row_idx][col_idx] = earliest;
                } else {
                    earliest = -1;
                    continue;
                }

                let mut best_earliest = earliest;
                for up_row in (0..=row_idx).rev() {
                    if earliest_continuous[up_row][col_idx] == -1 {
                        break;
                    }
                    // println!("adding up_row: {}", up_row);
                    best_earliest = best_earliest.max(earliest_continuous[up_row][col_idx]);
                    res += (col_idx as i32) - best_earliest + 1;
                }
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mat = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 13;
        assert_eq!(Solution::num_submat(mat), res);
    }

    #[test]
    fn test2() {
        let mat = [[0, 1, 1, 0], [0, 1, 1, 1], [1, 1, 1, 0]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 24;
        assert_eq!(Solution::num_submat(mat), res);
    }
}
