impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        if prices.is_empty() {
            return 0;
        }
        let (mut res, mut curr) = (0, 1);
        for idx in 1..prices.len() {
            if prices[idx] != prices[idx - 1] - 1 {
                res += (curr + 1) * curr / 2;
                curr = 0;
            }
            curr += 1;
        }
        res + (curr + 1) * curr / 2
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let prices = vec![3, 2, 1, 4];
        let res = 7;

        assert_eq!(Solution::get_descent_periods(prices), res);
    }

    #[test]
    fn test2() {
        let prices = vec![8, 6, 7, 7];
        let res = 4;

        assert_eq!(Solution::get_descent_periods(prices), res);
    }
}
