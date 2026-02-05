impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        let n = 26;
        #[inline]
        fn to_idx(x: char) -> usize {
            x as usize - 'a' as usize
        }
        const MAX: i64 = i64::MAX / 5;
        let mut dist = vec![vec![MAX; n]; n];

        for i in 0..n {
            dist[i][i] = 0;
        }

        let (mut min_let, mut max_let) = (usize::MAX, 0);
        for ((&src, &tgt), &cost) in std::iter::zip(std::iter::zip(&original, &changed), &cost) {
            let (x, y) = (to_idx(src), to_idx(tgt));
            min_let = min_let.min(x).min(y);
            max_let = max_let.max(x).max(y);
            dist[x][y] = dist[x][y].min(cost as i64);
        }

        for k in min_let..=max_let {
            for i in min_let..=max_let {
                for j in min_let..=max_let {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }

        let mut res = 0;
        for (src, tgt) in std::iter::zip(source.chars(), target.chars()) {
            let (x, y) = (to_idx(src), to_idx(tgt));
            let z = dist[x][y];
            if z == MAX {
                return -1;
            }
            res += z;
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let source = String::from("abcd");
        let target = String::from("acbe");
        let original = vec!['a', 'b', 'c', 'c', 'e', 'd'];
        let changed = vec!['b', 'c', 'b', 'e', 'b', 'e'];
        let cost = vec![2, 5, 5, 1, 2, 20];
        let res = 28;
        assert_eq!(
            Solution::minimum_cost(source, target, original, changed, cost),
            res
        );
    }
}
