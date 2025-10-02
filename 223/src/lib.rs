impl Solution {
    pub fn compute_area(
        ax1: i32,
        ay1: i32,
        ax2: i32,
        ay2: i32,
        bx1: i32,
        by1: i32,
        bx2: i32,
        by2: i32,
    ) -> i32 {
        let left = ax1.max(bx1);
        let right = ax2.min(bx2);
        let bottom = ay1.max(by1);
        let top = ay2.min(by2);
        return (ax2 - ax1) * (ay2 - ay1) + (bx2 - bx1) * (by2 - by1)
            - (right - left).max(0) * (top - bottom).max(0);
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) = (-3, 0, 3, 4, 0, -1, 9, 2);
        let res = 45;
        assert_eq!(
            Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2),
            res
        );
    }

    #[test]
    fn test2() {
        let (ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) = (-2, -2, 2, 2, -2, -2, 2, 2);
        let res = 16;
        assert_eq!(
            Solution::compute_area(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2),
            res
        );
    }
}
