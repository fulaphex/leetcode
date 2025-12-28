impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let cols = grid[0].len();
        let (mut first_negative, mut res) = (cols, 0);

        for row in &grid {
            while first_negative > 0 && row[first_negative - 1] < 0 {
                first_negative -= 1;
            }
            res += cols - first_negative;
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
        let grid = [
            [4, 3, 2, -1],
            [3, 2, 1, -1],
            [1, 1, -1, -2],
            [-1, -1, -2, -3],
        ]
        .into_iter()
        .map(Vec::from)
        .collect::<Vec<_>>();
        let res = 8;
        assert_eq!(Solution::count_negatives(grid), res);
    }
}
