use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        let mut to_teach = HashSet::new();
        let languages_known: Vec<HashSet<usize>> = Vec::from_iter(
            languages
                .into_iter()
                .map(|x| HashSet::from_iter(x.iter().map(|&y| (y - 1) as usize))),
        );

        for f in friendships {
            let (x, y) = (f[0] - 1, f[1] - 1);
            if languages_known[x as usize]
                .intersection(&languages_known[y as usize])
                .collect::<Vec<_>>()
                .is_empty()
            {
                to_teach.insert(x);
                to_teach.insert(y);
            }
        }

        if to_teach.is_empty() {
            return 0;
        }

        let mut known_count = HashMap::new();
        for &person in &to_teach {
            for l in &languages_known[person as usize] {
                *known_count.entry(l).or_insert(0) += 1;
            }
        }
        return to_teach.len() as i32 - known_count.values().max().unwrap();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let languages = vec![vec![1], vec![2], vec![1, 2]];
        let friendships = [[1, 2], [1, 3], [2, 3]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 1;
        assert_eq!(Solution::minimum_teachings(n, languages, friendships), res);
    }

    #[test]
    fn test2() {
        let n = 3;
        let languages = vec![vec![2], vec![1, 3], vec![1, 2], vec![3]];
        let friendships = [[1, 4], [1, 2], [3, 4], [2, 3]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2;
        assert_eq!(Solution::minimum_teachings(n, languages, friendships), res);
    }

    #[test]
    fn test3() {
        let n = 11;
        let languages = vec![
            vec![3, 11, 5, 10, 1, 4, 9, 7, 2, 8, 6],
            vec![5, 10, 6, 4, 8, 7],
            vec![6, 11, 7, 9],
            vec![11, 10, 4],
            vec![6, 2, 8, 4, 3],
            vec![9, 2, 8, 4, 6, 1, 5, 7, 3, 10],
            vec![7, 5, 11, 1, 3, 4],
            vec![3, 4, 11, 10, 6, 2, 1, 7, 5, 8, 9],
            vec![8, 6, 10, 2, 3, 1, 11, 5],
            vec![5, 11, 6, 4, 2],
        ];
        let friendships = [
            [7, 9],
            [3, 7],
            [3, 4],
            [2, 9],
            [1, 8],
            [5, 9],
            [8, 9],
            [6, 9],
            [3, 5],
            [4, 5],
            [4, 9],
            [3, 6],
            [1, 7],
            [1, 3],
            [2, 8],
            [2, 6],
            [5, 7],
            [4, 6],
            [5, 8],
            [5, 6],
            [2, 7],
            [4, 8],
            [3, 8],
            [6, 8],
            [2, 5],
            [1, 4],
            [1, 9],
            [1, 6],
            [6, 7],
        ]
        .into_iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 0;
        assert_eq!(Solution::minimum_teachings(n, languages, friendships), res);
    }
}
