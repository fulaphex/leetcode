impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let (row_count, col_count) = (matrix.len(), matrix[0].len());
        let mut cont_left = vec![vec![usize::MAX; col_count]; row_count];
        for r in 0..row_count {
            if matrix[r][0] == '1' {
                cont_left[r][0] = 0;
            }
        }

        let mut res = 0;
        for (ridx, row) in matrix.iter().enumerate() {
            for (cidx, field) in row.iter().enumerate() {
                if *field == '1' {
                    // update cont_left
                    if cidx > 0 && (cont_left[ridx][cidx - 1] != usize::MAX) {
                        cont_left[ridx][cidx] = cont_left[ridx][cidx - 1];
                    } else {
                        cont_left[ridx][cidx] = cidx;
                    }
                    // iterate up and check the result
                    let mut curr_left = cont_left[ridx][cidx];
                    for up in (0..=ridx).rev() {
                        curr_left = curr_left.max(cont_left[up][cidx]);
                        if curr_left > cidx {
                            break;
                        }
                        res = res.max((ridx - up + 1) * (cidx - curr_left + 1));
                    }
                }
            }
        }
        // print_mat(&matrix);
        // print_mat(&cont_left);
        return res as i32;
    }
}
struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix: Vec<_> = [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ]
        .into_iter()
        .map(|x| {
            x.into_iter()
                .map(|x| x.chars().next().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
        let res = 6;
        assert_eq!(Solution::maximal_rectangle(matrix), res);
    }
    #[test]
    fn test2() {
        let matrix: Vec<_> = [["1"]]
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let res = 1;
        assert_eq!(Solution::maximal_rectangle(matrix), res);
    }
    #[test]
    fn test3() {
        let matrix: Vec<_> = [["0"]]
            .into_iter()
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|x| x.chars().next().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        let res = 0;
        assert_eq!(Solution::maximal_rectangle(matrix), res);
    }
}
