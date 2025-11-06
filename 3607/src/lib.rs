use std::collections::BTreeSet;
use std::collections::VecDeque;

impl Solution {
    // union find + ordered set
    // pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    //     let c = c as usize;
    //     let mut representant = vec![0; c + 1];
    //     let mut connected_elements = vec![BTreeSet::new(); c + 1];
    //
    //     fn get_repr(node: usize, representant: &mut Vec<usize>) -> usize {
    //         if representant[node] == node {
    //             return node;
    //         }
    //         let res = get_repr(representant[node], representant);
    //         representant[node] = res;
    //         res
    //     }
    //
    //     for n in 1..=c {
    //         representant[n] = n;
    //         connected_elements[n].insert(n as i32);
    //     }
    //
    //     for conn in &connections {
    //         let (from, to) = (conn[0] as usize, conn[1] as usize);
    //
    //         let mut from_repr = get_repr(from, &mut representant);
    //         let mut to_repr = get_repr(to, &mut representant);
    //
    //         if to_repr == from_repr {
    //             continue;
    //         }
    //
    //         if connected_elements[from_repr].len() < connected_elements[to_repr].len() {
    //             (from_repr, to_repr) = (to_repr, from_repr);
    //         }
    //
    //         representant[to_repr] = from_repr;
    //
    //         if from_repr > to_repr {
    //             let (p1, p2) = connected_elements.split_at_mut(from_repr);
    //             p2[0].extend(p1[to_repr].iter());
    //         } else {
    //             let (p1, p2) = connected_elements.split_at_mut(to_repr);
    //             p1[from_repr].extend(p2[0].iter());
    //         }
    //     }
    //
    //     let mut res = vec![];
    //     for q in queries {
    //         let (type_, node) = (q[0], q[1] as usize);
    //         let repr_node = get_repr(node, &mut representant);
    //
    //         if type_ == 1 {
    //             let elements = &connected_elements[repr_node];
    //
    //             if elements.contains(&(node as i32)) {
    //                 res.push(node as i32);
    //             } else {
    //                 res.push(*connected_elements[repr_node].first().unwrap_or(&-1));
    //             }
    //         } else if type_ == 2 {
    //             connected_elements[repr_node].remove(&(node as i32));
    //         }
    //     }
    //     res
    // }
    // bfs + reverse processing of the queries
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let c = c as usize;
        let mut representant = vec![0; c + 1];
        // let mut connected_elements = vec![BTreeSet::new(); c + 1];
        let mut neighs = vec![vec![]; c + 1];
        for conn in &connections {
            let (from, to) = (conn[0] as usize, conn[1] as usize);
            neighs[from].push(to);
            neighs[to].push(from);
        }
        let mut curr_color = 0;

        let mut que = VecDeque::new();
        let mut visited = vec![false; c + 1];
        for x in 1..=c {
            if !visited[x] {
                curr_color += 1;
                que.push_back(x);
                visited[x] = true;
                representant[x] = curr_color;
            }
            while !que.is_empty() {
                let curr = que.pop_front().unwrap();
                for &n in &neighs[curr] {
                    if !visited[n] {
                        que.push_back(n);
                        visited[n] = true;
                        representant[n] = curr_color;
                    }
                }
            }
        }

        let mut offline_counter = vec![0; c + 1];
        for q in &queries {
            let (type_, node) = (q[0], q[1] as usize);

            if type_ == 2 {
                offline_counter[node] += 1;
            }
        }

        let mut lowest = vec![usize::MAX; curr_color + 1];
        for x in 0..=c {
            if offline_counter[x] == 0 {
                let repr = representant[x];
                lowest[repr] = lowest[repr].min(x);
            }
        }

        let mut res = vec![];
        for q in queries.iter().rev() {
            let (type_, node) = (q[0], q[1] as usize);
            let repr_node = representant[node];

            if type_ == 1 {
                if offline_counter[node] == 0 {
                    res.push(node as i32);
                } else if lowest[repr_node] == usize::MAX {
                    res.push(-1);
                } else {
                    res.push(lowest[repr_node] as i32);
                }
            } else if type_ == 2 {
                offline_counter[node] -= 1;
                if offline_counter[node] == 0 {
                    lowest[repr_node] = lowest[repr_node].min(node);
                }
            }
        }
        res.reverse();
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test1() {
        let c = 5;
        let connections: Vec<_> = [[1, 2], [2, 3], [3, 4], [4, 5]]
            .into_iter()
            .map(Vec::from)
            .collect();
        let queries: Vec<_> = [[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]]
            .into_iter()
            .map(Vec::from)
            .collect();
        let res = vec![3, 2, 3];

        assert_eq!(Solution::process_queries(c, connections, queries), res);
    }

    #[test]
    fn test2() {
        let string = fs::read_to_string("large_test").unwrap();
        let lines = string.lines().collect::<Vec<_>>();

        let c = lines[0].parse::<i32>().unwrap();
        let connections = lines[1][1..]
            .split("]")
            .map(|x| x.trim_start_matches(',').trim_start_matches('['))
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let queries = lines[2][1..]
            .split("]")
            .map(|x| x.trim_start_matches(',').trim_start_matches('['))
            .filter(|x| !x.is_empty())
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let res = lines[3][1..]
            .trim_end_matches(']')
            .split(",")
            .filter(|&x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        assert_eq!(Solution::process_queries(c, connections, queries), res);
    }
}
