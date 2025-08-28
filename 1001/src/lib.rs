use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = vec![];
        let (mut row_lamps, mut col_lamps, mut diag_lamps, mut diag2_lamps) = (
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
        );
        for lamp in &lamps {
            let (row, col) = (lamp[0], lamp[1]);
            let (diag, diag2) = (row - col, row + col);
            row_lamps
                .entry(row)
                .or_insert(HashSet::new())
                .insert((row, col));
            col_lamps
                .entry(col)
                .or_insert(HashSet::new())
                .insert((row, col));
            diag_lamps
                .entry(diag)
                .or_insert(HashSet::new())
                .insert((row, col));
            diag2_lamps
                .entry(diag2)
                .or_insert(HashSet::new())
                .insert((row, col));
        }

        for query in &queries {
            let (row, col) = (query[0], query[1]);
            let (diag, diag2) = (row - col, row + col);
            if row_lamps.get(&row).unwrap_or(&HashSet::new()).len() > 0 {
                res.push(1);
            } else if col_lamps.get(&col).unwrap_or(&HashSet::new()).len() > 0 {
                res.push(1);
            } else if diag_lamps.get(&diag).unwrap_or(&HashSet::new()).len() > 0 {
                res.push(1);
            } else if diag2_lamps.get(&diag2).unwrap_or(&HashSet::new()).len() > 0 {
                res.push(1);
            } else {
                res.push(0);
            }
            for r in (row - 1)..=(row + 1) {
                for c in (col - 1)..=(col + 1) {
                    let (d, d2) = (r - c, r + c);
                    Self::rem_light(&(r, c), r, &mut row_lamps);
                    Self::rem_light(&(r, c), c, &mut col_lamps);
                    Self::rem_light(&(r, c), d, &mut diag_lamps);
                    Self::rem_light(&(r, c), d2, &mut diag2_lamps);
                }
            }
        }

        return res;
    }

    fn rem_light(lamp: &(i32, i32), row: i32, row_lamps: &mut HashMap<i32, HashSet<(i32, i32)>>) {
        let lamps = row_lamps.get_mut(&row);
        if lamps.is_some() {
            lamps.unwrap().remove(lamp);
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
}
