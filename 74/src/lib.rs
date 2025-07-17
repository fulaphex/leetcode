struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        if matrix.len() == 0 {
            return false;
        }
        if matrix[0][0] > target {
            return false;
        }
        let first_col = (0..matrix.len())
            .map(|idx| matrix[idx][0])
            .collect::<Vec<_>>();
        println!("target: {}", target);
        println!("first_col: {:?}", first_col);
        println!("matrix: {:?}", matrix);
        let first_res = first_col.binary_search(&target);
        let row_idx = match first_res {
            Ok(val) => val,
            Err(val) => val - 1,
        };
        println!("selected_row: {:?}", matrix[row_idx]);
        println!("{:?}", matrix[row_idx].binary_search(&target));
        return match matrix[row_idx].binary_search(&target) {
            Ok(_) => true,
            Err(_) => false,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 3;
        assert!(Solution::search_matrix(matrix, target));
    }

    #[test]
    fn test2() {
        let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
        let target = 0;
        assert!(!Solution::search_matrix(matrix, target));
    }
}
