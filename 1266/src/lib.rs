impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        points
            .windows(2)
            .map(|ps| (ps[1][0] - ps[0][0]).abs().max((ps[1][1] - ps[0][1]).abs()))
            .sum()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let points = [[1, 1], [3, 4], [-1, 0]]
            .into_iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 7;
        assert_eq!(Solution::min_time_to_visit_all_points(points), res);
    }
}
