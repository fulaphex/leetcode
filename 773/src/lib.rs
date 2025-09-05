use std::collections::{HashSet, VecDeque};

impl Solution {
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn check(board: &Vec<Vec<i32>>, (cx, cy): (usize, usize), (dx, dy): (i32, i32)) -> bool {
        let (nx, ny) = (cx as i32 + dx, cy as i32 + dy);
        return (0 <= nx) && (nx < board.len() as i32) && (0 <= ny) && (ny < board[0].len() as i32);
    }

    fn get_neighs(board: &Vec<Vec<i32>>, (cx, cy): (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (dx, dy) in Self::DIRS {
            if Self::check(board, (cx, cy), (dx, dy)) {
                res.push(((cx as i32 + dx) as usize, (cy as i32 + dy) as usize));
            }
        }
        return res;
    }

    fn repr(board: &Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        for row in board {
            for f in row {
                res = 6 * res + f;
            }
        }
        return res;
    }

    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        let dest = vec![vec![1, 2, 3], vec![4, 5, 0]];
        let dest_repr = Self::repr(&dest);

        let mut empty = (usize::MAX, usize::MAX);
        for (ridx, row) in board.iter().enumerate() {
            for (cidx, &f) in row.iter().enumerate() {
                if f == 0 {
                    empty = (ridx, cidx);
                }
            }
        }

        let mut visited = HashSet::new();
        let rep = Self::repr(&board);
        let mut que = VecDeque::from([(board, empty, 0, rep)]);

        while !que.is_empty() {
            let (board, empty, dist, rep) = que.pop_front().unwrap();
            if rep == dest_repr {
                return dist;
            }

            visited.insert(rep);
            for (nx, ny) in Self::get_neighs(&board, empty) {
                let mut new_board = board.clone();
                (new_board[empty.0][empty.1], new_board[nx][ny]) =
                    (new_board[nx][ny], new_board[empty.0][empty.1]);

                let new_repr = Self::repr(&new_board);

                if !visited.contains(&new_repr) {
                    que.push_back((new_board, (nx, ny), dist + 1, new_repr));
                }
            }
        }
        return -1;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let board = vec![vec![1, 2, 3], vec![4, 5, 0]];
        let res = 0;
        // println!("repr: {}", repr(&board));
        assert_eq!(Solution::sliding_puzzle(board), res);
    }

    #[test]
    fn test2() {
        let board = vec![vec![1, 2, 3], vec![5, 4, 0]];
        let res = -1;
        // println!("repr: {}", repr(&board));
        assert_eq!(Solution::sliding_puzzle(board), res);
    }
}
