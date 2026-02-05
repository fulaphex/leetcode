#[derive(PartialEq)]
enum TREND {
    INCREASING,
    DECREASING,
    SAME,
}

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        let get_trend = |x: i32, y: i32| match x.cmp(&y) {
            std::cmp::Ordering::Less => TREND::INCREASING,
            std::cmp::Ordering::Greater => TREND::DECREASING,
            std::cmp::Ordering::Equal => TREND::SAME,
        };
        let mut prev = nums[0];
        let mut trend = get_trend(nums[0], nums[1]);
        let mut trends = vec![];
        for &x in nums.iter().skip(1) {
            let new_trend = get_trend(prev, x);
            if trend != new_trend {
                trends.push(trend);
            }
            trend = new_trend;
            prev = x;
        }
        trends.push(trend);

        trends == vec![TREND::INCREASING, TREND::DECREASING, TREND::INCREASING]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, 5, 4, 2, 6];

        let res = true;
        assert_eq!(Solution::is_trionic(nums), res);
    }
}
