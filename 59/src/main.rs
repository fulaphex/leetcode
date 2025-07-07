struct Solution {}

#[derive(PartialEq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        // pub fn generate_matrix(n: i32) -> () {
        let un = n.try_into().unwrap();
        let mut res = vec![vec![0; un]; un];
        let (mut x, mut y): (usize, usize) = (0, 0);
        println!("row: {:?}", res);
        res[0][0] = 1;
        println!("row: {:?}", res);
        let mut direction = Direction::Right;
        let mut consecutive_turns = 0;
        loop {
            println!("{} {} {}", x, y, res[y][x]);
            if consecutive_turns > 3 {
                break;
            }
            if direction == Direction::Right {
                if (x + 1 < un) && res[y][x + 1] == 0 {
                    // try right
                    res[y][x + 1] = res[y][x] + 1;
                    x += 1;
                    consecutive_turns = 0;
                } else {
                    direction = Direction::Down;
                    consecutive_turns += 1;
                }
            } else if direction == Direction::Down {
                if (y + 1 < un) && res[y + 1][x] == 0 {
                    // try right
                    res[y + 1][x] = res[y][x] + 1;
                    y += 1;
                    consecutive_turns = 0;
                } else {
                    direction = Direction::Left;
                    consecutive_turns += 1;
                }
            } else if direction == Direction::Left {
                if (x > 0) && res[y][x - 1] == 0 {
                    // try right
                    res[y][x - 1] = res[y][x] + 1;
                    x -= 1;
                    consecutive_turns = 0;
                } else {
                    direction = Direction::Up;
                    consecutive_turns += 1;
                }
            } else if direction == Direction::Up {
                if (y > 0) && res[y - 1][x] == 0 {
                    // try right
                    res[y - 1][x] = res[y][x] + 1;
                    y -= 1;
                    consecutive_turns = 0;
                } else {
                    direction = Direction::Right;
                    consecutive_turns += 1;
                }
            }
        }
        println!("row: {:?}", res);
        return res;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_1() {
        let res = Solution::generate_matrix(1);
        assert_eq!(res, Vec::from([[1]]));
    }

    #[test]
    fn test_simple() {
        let res = Solution::generate_matrix(3);
        assert_eq!(res, Vec::from([[1, 2, 3], [8, 9, 4], [7, 6, 5]]));
    }

    #[test]
    fn test_simple5() {
        let res = Solution::generate_matrix(5);
        assert_eq!(
            res,
            Vec::from([
                [1, 2, 3, 4, 5],
                [16, 17, 18, 19, 6],
                [15, 24, 25, 20, 7],
                [14, 23, 22, 21, 8],
                [13, 12, 11, 10, 9]
            ])
        );
    }
}
