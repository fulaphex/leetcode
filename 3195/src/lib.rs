struct Solution {}

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let (mut minx, mut miny, mut maxx, mut maxy) = (i32::MAX, i32::MAX, -1, -1);
        for row in 0..grid.len() as i32 {
            for col in 0..grid[0].len() as i32 {
                if grid[row as usize][col as usize] == 1 {
                    minx = minx.min(row);
                    miny = miny.min(col);
                    maxx = maxx.max(row);
                    maxy = maxy.max(col);
                }
            }
        }
        return (maxx - minx + 1) * (maxy - miny + 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [[0, 1, 0], [1, 0, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 6;
        assert_eq!(Solution::minimum_area(grid), res);
    }

    #[test]
    fn test2() {
        let grid = [[1, 0], [0, 0]].iter().map(|x| Vec::from(x)).collect();
        let res = 1;
        assert_eq!(Solution::minimum_area(grid), res);
    }
}
