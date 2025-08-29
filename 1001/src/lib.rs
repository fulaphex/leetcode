use std::{
    collections::{HashMap, HashSet},
    ops::{AddAssign, SubAssign},
};

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        fn get_diags(row: i32, col: i32) -> (i32, i32) {
            return (row - col, row + col);
        }

        let mut res = vec![];
        let (mut row_lamps, mut col_lamps, mut diag_lamps, mut diag2_lamps) = (
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        );
        let mut lamp_set: HashSet<(i32, i32)> = lamps.into_iter().map(|x| (x[0], x[1])).collect();
        for (row, col) in &lamp_set {
            let (diag, diag2) = get_diags(*row, *col);
            row_lamps.entry(*row).or_insert(0).add_assign(1);
            col_lamps.entry(*col).or_insert(0).add_assign(1);
            diag_lamps.entry(diag).or_insert(0).add_assign(1);
            diag2_lamps.entry(diag2).or_insert(0).add_assign(1);
        }

        for query in &queries {
            let (row, col) = (query[0], query[1]);
            let (diag, diag2) = get_diags(row, col);
            if row_lamps.get(&row).unwrap_or(&0) > &0
                || col_lamps.get(&col).unwrap_or(&0) > &0
                || diag_lamps.get(&diag).unwrap_or(&0) > &0
                || diag2_lamps.get(&diag2).unwrap_or(&0) > &0
            {
                res.push(1);
            } else {
                res.push(0);
            }
            for r in (row - 1)..=(row + 1) {
                for c in (col - 1)..=(col + 1) {
                    if !lamp_set.remove(&(r, c)) {
                        continue;
                    }
                    let (d, d2) = get_diags(r, c);
                    Self::rem_light(&(r, c), r, &mut row_lamps);
                    Self::rem_light(&(r, c), c, &mut col_lamps);
                    Self::rem_light(&(r, c), d, &mut diag_lamps);
                    Self::rem_light(&(r, c), d2, &mut diag2_lamps);
                }
            }
        }

        return res;
    }

    fn rem_light(lamp: &(i32, i32), row: i32, row_lamps: &mut HashMap<i32, i32>) {
        let count = row_lamps.get_mut(&row);
        if count.is_some() {
            count.unwrap().sub_assign(1);
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        let lamps = [[0, 0], [4, 4]].iter().map(|x| Vec::from(x)).collect();
        let queries = [[1, 1], [1, 0]].iter().map(|x| Vec::from(x)).collect();
        let res = vec![1, 0];
        assert_eq!(Solution::grid_illumination(n, lamps, queries), res);
    }

    #[test]
    fn test2() {
        let n = 5;
        let lamps = [[0, 0], [4, 4]].iter().map(|x| Vec::from(x)).collect();
        let queries = [[1, 1], [1, 1]].iter().map(|x| Vec::from(x)).collect();
        let res = vec![1, 1];
        assert_eq!(Solution::grid_illumination(n, lamps, queries), res);
    }

    #[test]
    fn test3() {
        let n = 5;
        let lamps = [[0, 0], [0, 4]].iter().map(|x| Vec::from(x)).collect();
        let queries = [[0, 4], [0, 1], [1, 4]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = vec![1, 1, 0];
        assert_eq!(Solution::grid_illumination(n, lamps, queries), res);
    }

    #[test]
    fn test4() {
        let n = 6;
        let lamps = [
            [2, 5],
            [4, 2],
            [0, 3],
            [0, 5],
            [1, 4],
            [4, 2],
            [3, 3],
            [1, 0],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let queries = [[4, 3], [3, 1], [5, 3], [0, 5], [4, 4], [3, 3]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = vec![1, 0, 1, 1, 0, 1];
        assert_eq!(Solution::grid_illumination(n, lamps, queries), res);
    }
}
