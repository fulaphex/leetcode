struct Solution {}

impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut res = vec![];
        let (mut row, mut col) = (0, 0);
        let down_left = (1, -1);
        let up_right = (-1, 1);
        let mut dir = up_right;
        loop {
            res.push(mat[row][col]);

            if row == m - 1 && col == n - 1 {
                break;
            }

            if dir == up_right && col == n - 1 {
                row += 1;
                dir = down_left;
            } else if dir == down_left && row == m - 1 {
                col += 1;
                dir = up_right;
            } else if dir == up_right && row == 0 {
                col += 1;
                dir = down_left;
            } else if dir == down_left && col == 0 {
                row += 1;
                dir = up_right;
            } else {
                row = (row as i32 + dir.0) as usize;
                col = (col as i32 + dir.1) as usize;
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
        let mat = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = vec![1, 2, 4, 7, 5, 3, 6, 8, 9];
        assert_eq!(Solution::find_diagonal_order(mat), res);
    }

    #[test]
    fn test2() {
        let mat = [[1, 2], [3, 4]].into_iter().map(|x| Vec::from(x)).collect();
        let res = vec![1, 2, 3, 4];
        assert_eq!(Solution::find_diagonal_order(mat), res);
    }

    #[test]
    fn test3() {
        let mat = [[1, 2, 3], [4, 5, 6]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = vec![1, 2, 4, 5, 3, 6];
        assert_eq!(Solution::find_diagonal_order(mat), res);
    }
}
