use std::collections::HashMap;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let mut point_count_y = HashMap::new();
        for point in points {
            *point_count_y.entry(point[1]).or_insert(0) += 1;
        }

        let (mut curr_sum, mut res) = (0, 0);
        for v in point_count_y.values() {
            let x = v * (v - 1) / 2;
            res = (res + x * curr_sum) % MOD;
            curr_sum = (curr_sum + x) % MOD;
        }

        res as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let points = [[1, 0], [2, 0], [3, 0], [2, 2], [3, 2]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 3;

        assert_eq!(Solution::count_trapezoids(points), res);
    }
}
