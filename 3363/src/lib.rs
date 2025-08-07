struct Solution {}

impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![-1; fruits.len()]; fruits.len()];

        // start conditions
        dp[0][0] = fruits[0][0];
        dp[fruits.len() - 1][0] = fruits[fruits.len() - 1][0];
        dp[0][fruits.len() - 1] = fruits[0][fruits.len() - 1];

        for row in 1..fruits.len() {
            for col in 1..fruits.len() {
                if row == col {
                    if row == 0 {
                        continue;
                    } else if row == fruits.len() - 1 {
                        dp[row][col] = fruits[row][col]
                            + dp[row - 1][col - 1]
                            + dp[row - 1][col]
                            + dp[row][col - 1];
                        continue;
                    }
                    dp[row][col] = fruits[row][col] + dp[row - 1][col - 1];
                } else if row < col {
                    // top corner
                    let prev: Vec<_> = [(row - 1, col - 1), (row - 1, col), (row - 1, col + 1)]
                        .into_iter()
                        .filter(|(_, y)| *y < fruits.len())
                        .filter(|(x, y)| dp[*x][*y] != -1)
                        .collect();
                    let max = prev.iter().map(|(x, y)| dp[*x][*y]).max();
                    if max.is_some() {
                        dp[row][col] = fruits[row][col] + max.unwrap();
                    }
                    // bottom corner
                    let (row, col) = (col, row);
                    let prev: Vec<_> = [(row - 1, col - 1), (row, col - 1), (row + 1, col - 1)]
                        .into_iter()
                        .filter(|(x, _)| *x < fruits.len())
                        .filter(|(x, y)| dp[*x][*y] != -1)
                        .collect();
                    let max = prev.iter().map(|(x, y)| dp[*x][*y]).max();
                    if max.is_some() {
                        dp[row][col] = fruits[row][col] + max.unwrap();
                    }
                }
            }
        }

        return dp[fruits.len() - 1][fruits.len() - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let fruits: Vec<Vec<i32>> = [
            [1, 2, 3, 4],
            [5, 6, 8, 7],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 100;
        assert_eq!(Solution::max_collected_fruits(fruits), res);
    }

    #[test]
    fn test2() {
        let fruits: Vec<Vec<i32>> = [[1, 1], [1, 1]].iter().map(|x| Vec::from(x)).collect();
        let res = 4;
        assert_eq!(Solution::max_collected_fruits(fruits), res);
    }

    #[test]
    fn test3() {
        let fruits: Vec<Vec<i32>> = [
            [16, 9, 1, 2, 3],
            [5, 14, 12, 1, 13],
            [3, 12, 20, 3, 10],
            [12, 1, 16, 5, 13],
            [2, 19, 11, 9, 10],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 150;
        assert_eq!(Solution::max_collected_fruits(fruits), res);
    }
}
