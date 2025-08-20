struct Solution {}

fn print_mat<T>(arr: &Vec<Vec<T>>)
where
    T: std::fmt::Debug,
{
    for row in arr.iter() {
        println!("{:?}", row);
    }
}

impl Solution {
    pub fn count_squares(mut matrix: Vec<Vec<i32>>) -> i32 {
        let (row_count, col_count) = (matrix.len(), matrix[0].len());
        let mut res = 0;

        for row_idx in 0..row_count {
            for col_idx in 0..col_count {
                if matrix[row_idx][col_idx] == 0 {
                    continue;
                }
                if row_idx != 0 && col_idx != 0 {
                    matrix[row_idx][col_idx] = [
                        matrix[row_idx - 1][col_idx - 1],
                        (matrix[row_idx - 1][col_idx]),
                        (matrix[row_idx][col_idx - 1]),
                    ]
                    .iter()
                    .min()
                    .unwrap()
                        + 1;
                }
                res += matrix[row_idx][col_idx];
            }
        }
        // print_mat(&matrix);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix: Vec<Vec<i32>> = [[0, 1, 1, 1], [1, 1, 1, 1], [0, 1, 1, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 15;
        assert_eq!(Solution::count_squares(matrix), res);
        // assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let matrix: Vec<Vec<i32>> = [[1, 0, 1], [1, 1, 0], [1, 1, 0]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 7;
        assert_eq!(Solution::count_squares(matrix), res);
        // assert_eq!(result, 4);
    }
}
