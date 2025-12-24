impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let total_apples = apple.iter().sum::<i32>();
        capacity.sort_unstable();
        let (mut cnt, mut sum) = (0, 0);
        for c in capacity.iter().rev() {
            cnt += 1;
            sum += c;
            if sum >= total_apples {
                return cnt;
            }
        }
        unreachable!();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let apple = vec![1, 3, 2];
        let capacity = vec![4, 3, 1, 5, 2];
        let res = 2;

        assert_eq!(Solution::minimum_boxes(apple, capacity), res);
    }
}
