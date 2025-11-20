use std::collections::HashMap;

impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;

        // let mut begins = vec![vec![]; n + 1];
        // let mut ends = vec![vec![]; n + 1];

        let mut begins = HashMap::new();
        let mut ends = HashMap::new();

        for q in queries {
            let (start_row, start_col, end_row, end_col) =
                (q[0] as usize, q[1] as usize, q[2] as usize, q[3] as usize);
            begins
                .entry(start_row)
                .or_insert(vec![])
                .push((start_col, end_col));
            ends.entry(end_row + 1)
                .or_insert(vec![])
                .push((start_col, end_col));
            // begins[start_row].push((start_col, end_col));
            // ends[end_row + 1].push((start_col, end_col));
        }

        let mut res = vec![vec![0; n]; n];
        let mut row_mods = vec![0; n + 1];

        for (row_idx, row) in res.iter_mut().enumerate() {
            let mut val = 0;
            for &(start, end) in begins.get(&row_idx).unwrap_or(&vec![]) {
                // for &(start, end) in &begins[row_idx] {
                row_mods[start] += 1;
                row_mods[end + 1] -= 1;
            }
            for &(start, end) in ends.get(&row_idx).unwrap_or(&vec![]) {
                // for &(start, end) in &ends[row_idx] {
                row_mods[start] -= 1;
                row_mods[end + 1] += 1;
            }
            for col_idx in 0..row.len() {
                val += row_mods[col_idx];
                row[col_idx] = val;
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
    fn test1() {
        let n = 3;
        let queries: Vec<_> = [[1, 1, 2, 2], [0, 0, 1, 1]].iter().map(Vec::from).collect();
        let res: Vec<_> = [[1, 1, 0], [1, 2, 1], [0, 1, 1]]
            .iter()
            .map(Vec::from)
            .collect();
        assert_eq!(Solution::range_add_queries(n, queries), res);
    }
}
