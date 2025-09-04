use std::cmp::Ordering;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        match (x-z).abs().cmp(&(y-z).abs()) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => 2,
        }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (x, y, z) = (2, 7, 4);
        let res = 1;
        assert_eq!(Solution::find_closest(x, y, z), res);
    }

    #[test]
    fn test2() {
        let (x, y, z) = (2, 5, 6);
        let res = 2;
        assert_eq!(Solution::find_closest(x, y, z), res);
    }

    #[test]
    fn test3() {
        let (x, y, z) = (1, 5, 3);
        let res = 0;
        assert_eq!(Solution::find_closest(x, y, z), res);
    }
}
