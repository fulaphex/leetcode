use std::{cmp::Reverse, collections::BinaryHeap};

impl Solution {
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut neigh_costs: Vec<Vec<(usize, i32)>> = vec![vec![]; n];
        for edge in edges {
            let (start, end, cost) = (edge[0] as usize, edge[1] as usize, edge[2]);

            neigh_costs[start].push((end, cost));
            neigh_costs[end].push((start, 2 * cost));
        }

        let mut visited = vec![false; n];
        let mut que = BinaryHeap::from([(Reverse(0), 0)]);

        while let Some((Reverse(cost), node)) = que.pop() {
            if node == n - 1 {
                return cost;
            }

            if visited[node] {
                continue;
            }

            visited[node] = true;
            for &(x, c) in &neigh_costs[node] {
                if visited[x] {
                    continue;
                }
                let new_cost = cost + c;
                que.push((Reverse(new_cost), x));
            }
        }

        -1
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let edges = [[0, 1, 3], [3, 1, 1], [2, 3, 4], [0, 2, 2]]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 5;
        assert_eq!(Solution::min_cost(n, edges), res);
    }
}
