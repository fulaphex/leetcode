use std::collections::{HashSet, VecDeque};

impl Solution {
    fn build_graph(k: i32, conditions: &Vec<Vec<i32>>) -> (Vec<HashSet<usize>>, Vec<usize>) {
        let uk = k as usize;
        let mut children = vec![HashSet::new(); uk];
        let mut parents = vec![0; uk];

        for cond in HashSet::<&Vec<i32>>::from_iter(conditions.into_iter()) {
            let (x, y) = (cond[0] as usize, cond[1] as usize);
            children[x - 1].insert(y - 1);
            parents[y - 1] += 1;
        }

        return (children, parents);
    }
    fn topo_sort(
        uk: usize,
        mut parents: Vec<usize>,
        children: Vec<HashSet<usize>>,
    ) -> Option<Vec<i32>> {
        let mut empty_parents: VecDeque<_> = parents
            .iter()
            .enumerate()
            .filter(|&(_k, &v)| v == 0)
            .map(|(k, _v)| k)
            .collect();

        let mut acc = vec![];

        while empty_parents.len() > 0 {
            let par = empty_parents.pop_front().unwrap();
            acc.push(par as i32);

            for &c in &children[par] {
                parents[c] -= 1;
                if parents[c] == 0 {
                    empty_parents.push_back(c);
                }
            }
        }

        return if acc.len() == uk { Some(acc) } else { None };
    }

    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let uk = k as usize;
        let (row_children, row_parents) = Self::build_graph(k, &row_conditions);
        let row_vals = Self::topo_sort(uk, row_parents, row_children);

        let (col_children, col_parents) = Self::build_graph(k, &col_conditions);
        let col_vals = Self::topo_sort(uk, col_parents, col_children);

        if row_vals.is_none() || col_vals.is_none() {
            return vec![];
        }

        let mut row_idx = vec![usize::MAX; uk];
        let mut col_idx = vec![usize::MAX; uk];

        for (idx, v) in row_vals.unwrap().into_iter().enumerate() {
            row_idx[v as usize] = idx;
        }

        for (idx, v) in col_vals.unwrap().into_iter().enumerate() {
            col_idx[v as usize] = idx;
        }

        let mut grid = vec![vec![0; uk]; uk];

        for val in 0..uk {
            grid[row_idx[val]][col_idx[val]] = (val as i32) + 1;
        }

        return grid;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let k = 3;
        let row_conditions: Vec<Vec<i32>> =
            [[1, 2], [3, 2]].into_iter().map(|x| Vec::from(x)).collect();
        let col_conditions: Vec<Vec<i32>> =
            [[2, 1], [3, 2]].into_iter().map(|x| Vec::from(x)).collect();
        let res: Vec<Vec<i32>> = [[0, 0, 1], [3, 0, 0], [0, 2, 0]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        assert_eq!(
            Solution::build_matrix(k, row_conditions, col_conditions),
            res
        );
    }

    #[test]
    fn test2() {
        let k = 3;
        let row_conditions: Vec<Vec<i32>> = [[1, 2], [2, 3], [3, 1], [2, 3]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let col_conditions: Vec<Vec<i32>> = [[2, 1]].into_iter().map(|x| Vec::from(x)).collect();
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::build_matrix(k, row_conditions, col_conditions),
            res
        );
    }
}
