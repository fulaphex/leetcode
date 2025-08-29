use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    fn build_graph(
        k: i32,
        conditions: &Vec<Vec<i32>>,
    ) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
        let uk = k as usize;
        let mut children = vec![HashSet::new(); uk];
        let mut parents = vec![HashSet::new(); uk];

        for cond in conditions {
            let (x, y) = (cond[0] as usize, cond[1] as usize);
            children[x - 1].insert(y - 1);
            parents[y - 1].insert(x - 1);
        }

        return (children, parents);
    }
    fn topo_sort(
        k: i32,
        mut parents: Vec<HashSet<usize>>,
        children: Vec<HashSet<usize>>,
    ) -> Option<Vec<i32>> {
        let mut empty_parents: VecDeque<_> = parents
            .iter()
            .enumerate()
            .filter(|(_k, v)| v.len() == 0)
            .map(|(k, _v)| k)
            .collect();

        let mut acc = vec![];

        while empty_parents.len() > 0 {
            let par = empty_parents.pop_front().unwrap();
            acc.push(par as i32);

            for c in &children[par] {
                parents[*c as usize].remove(&par);
                if parents[*c as usize].len() == 0 {
                    empty_parents.push_back(*c);
                }
            }
        }

        return if acc.len() == (k as usize) {
            Some(acc)
        } else {
            None
        };
    }

    pub fn build_matrix(
        k: i32,
        row_conditions: Vec<Vec<i32>>,
        col_conditions: Vec<Vec<i32>>,
    ) -> Vec<Vec<i32>> {
        let row_graph = Self::build_graph(k, &row_conditions);
        let row_vals = Self::topo_sort(k, row_graph.1, row_graph.0);

        let col_graph = Self::build_graph(k, &col_conditions);
        let col_vals = Self::topo_sort(k, col_graph.1, col_graph.0);

        if row_vals.is_none() || col_vals.is_none() {
            return vec![];
        }

        let mut row_map: HashMap<i32, usize> = HashMap::new();
        let mut col_map: HashMap<i32, usize> = HashMap::new();

        for (idx, v) in row_vals.unwrap().iter().enumerate() {
            row_map.insert(*v, idx);
        }

        for (idx, v) in col_vals.unwrap().iter().enumerate() {
            col_map.insert(*v, idx);
        }

        let uk = k as usize;
        let mut grid = vec![vec![0; uk]; uk];

        for val in 0..k {
            grid[*row_map.get(&val).unwrap()][*col_map.get(&val).unwrap()] = val + 1;
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
        let row_conditions: Vec<Vec<i32>> = [[1, 2], [3, 2]].iter().map(|x| Vec::from(x)).collect();
        let col_conditions: Vec<Vec<i32>> = [[2, 1], [3, 2]].iter().map(|x| Vec::from(x)).collect();
        let res: Vec<Vec<i32>> = [[0, 0, 1], [3, 0, 0], [0, 2, 0]]
            .iter()
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
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let col_conditions: Vec<Vec<i32>> = [[2, 1]].iter().map(|x| Vec::from(x)).collect();
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(
            Solution::build_matrix(k, row_conditions, col_conditions),
            res
        );
    }
}
