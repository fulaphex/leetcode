struct Solution {}

impl Solution {
    fn min_area(grid: &[&[i32]]) -> i32 {
        let (mut minx, mut miny, mut maxx, mut maxy) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        assert!(grid.len() > 0);
        assert!(grid[0].len() > 0);
        for row_idx in 0..grid.len() {
            for col_idx in 0..grid[0].len() {
                if grid[row_idx][col_idx] == 1 {
                    minx = minx.min(row_idx as i32);
                    miny = miny.min(col_idx as i32);

                    maxx = maxx.max(row_idx as i32);
                    maxy = maxy.max(col_idx as i32);
                }
            }
        }

        return if minx == i32::MAX {
            1
        } else {
            (maxx - minx + 1) * (maxy - miny + 1)
        };
    }

    fn split(num_splits: i8, grid: &[&[i32]]) -> i32 {
        if num_splits == 0 {
            return Self::min_area(&grid);
        }

        let mut res = i32::MAX;

        // try splitting by rows
        for row_split in 1..grid.len() {
            let sub_grid = &grid[..row_split];
            let other_subgrid = &grid[row_split..];

            res = res.min(Self::min_area(sub_grid) + Self::split(num_splits - 1, other_subgrid));
            res = res.min(Self::min_area(other_subgrid) + Self::split(num_splits - 1, sub_grid));
        }

        // try splitting by columns
        for col_split in 1..grid[0].len() {
            let sub_grid = grid.iter().map(|&x| &x[..col_split]).collect::<Vec<_>>();
            let other_subgrid = grid.iter().map(|&x| &x[col_split..]).collect::<Vec<_>>();

            res = res.min(
                Self::min_area(sub_grid.as_slice())
                    + Self::split(num_splits - 1, other_subgrid.as_slice()),
            );
            res = res.min(
                Self::min_area(other_subgrid.as_slice())
                    + Self::split(num_splits - 1, sub_grid.as_slice()),
            );
        }

        return res;
    }

    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let x = grid.iter().map(|x| x.as_slice()).collect::<Vec<_>>();
        return Self::split(2, x.as_slice());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [[1, 0, 1], [1, 1, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 5;

        assert_eq!(Solution::minimum_sum(grid), res);
    }

    #[test]
    fn test2() {
        let grid = [[1, 0, 1, 0], [0, 1, 0, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 5;

        assert_eq!(Solution::minimum_sum(grid), res);
    }

    #[test]
    fn test3() {
        let grid = [[1, 1], [1, 0]].iter().map(|x| Vec::from(x)).collect();
        let res = 3;

        assert_eq!(Solution::minimum_sum(grid), res);
    }
}
