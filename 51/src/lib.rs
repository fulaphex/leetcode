struct Solution {}

impl Solution {
    fn change_queen(col: usize, taken: &mut [Vec<i8>], delta: i8) {
        for x in taken[0].iter_mut() {
            *x += delta;
        }

        for row_idx in 1..taken.len() {
            taken[row_idx][col] += delta;
            if col >= row_idx {
                taken[row_idx][col - row_idx] += delta;
            }
            if col + row_idx < taken[0].len() {
                taken[row_idx][col + row_idx] += delta;
            }
        }
    }

    fn add_queen(col: usize, taken: &mut [Vec<i8>]) {
        Self::change_queen(col, taken, 1);
    }

    fn remove_queen(col: usize, taken: &mut [Vec<i8>]) {
        Self::change_queen(col, taken, -1);
    }

    fn recurse(n: usize, taken: &mut [Vec<i8>], res: &mut Vec<Vec<i32>>, acc: &mut Vec<i32>) {
        if taken.len() == 0 {
            res.push(acc.clone());
            return;
        }
        for col in 0..n {
            if taken[0][col] > 0 {
                continue;
            }
            Self::add_queen(col, taken);
            acc.push(col as i32);

            Self::recurse(n, &mut taken[1..], res, acc);

            Self::remove_queen(col, taken);
            acc.pop();
        }
    }
    fn parse(n: usize, queens: &Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        for &q in queens {
            res.push(String::from_iter(
                (0..n as i32).map(|x| if x == q { 'Q' } else { '.' }),
            ));
        }
        return res;
    }

    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let un = n as usize;
        let res: Vec<Vec<String>> = vec![];
        let mut taken = vec![vec![0; un]; un];
        let mut raw_res = vec![];
        Self::recurse(un, &mut taken, &mut raw_res, &mut vec![]);
        return raw_res.iter().map(|x| Self::parse(un, x)).collect();
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let un = n as usize;
        let mut taken = vec![vec![0; un]; un];
        let mut res = vec![];
        Self::recurse(un, &mut taken, &mut res, &mut vec![]);
        return res.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 4;
        let res = vec![
            [".Q..", "...Q", "Q...", "..Q."].map(|x| String::from(x)),
            ["..Q.", "Q...", "...Q", ".Q.."].map(|x| String::from(x)),
        ];
        assert_eq!(Solution::solve_n_queens(n), res);
        assert_eq!(Solution::total_n_queens(n), 2);
    }

    #[test]
    fn test2() {
        let n = 1;
        let res = vec![["Q"].map(|x| String::from(x))];
        assert_eq!(Solution::total_n_queens(n), 1);
    }

    #[test]
    fn test3() {
        let n = 9;
        assert_eq!(Solution::total_n_queens(n), 352);
    }
}
