impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut res, mut left_idx, mut right_idx) = (0, 0, height.len() - 1);
        while left_idx < right_idx {
            let (left, right) = (height[left_idx], height[right_idx]);
            let area = left.min(right) * (right_idx - left_idx) as i32;
            res = res.max(area);
            if left < right {
                left_idx += 1;
            } else {
                right_idx -= 1;
            }
        }
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let res = 49;
        assert_eq!(Solution::max_area(height), res);
    }
}
