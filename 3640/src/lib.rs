#[derive(Debug, PartialEq)]
enum TREND {
    INCREASING,
    DECREASING,
    SAME,
}

impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let nums = nums.into_iter().map(|x| x as i64).collect::<Vec<_>>();
        let get_trend = |x: i64, y: i64| match x.cmp(&y) {
            std::cmp::Ordering::Less => TREND::INCREASING,
            std::cmp::Ordering::Greater => TREND::DECREASING,
            std::cmp::Ordering::Equal => TREND::SAME,
        };

        let mut current_trend = get_trend(nums[0], nums[1]);
        let mut new_trend;
        let mut start = 0;
        let mut prev = nums[0];
        let mut ranges = vec![];
        for (idx, &x) in nums.iter().enumerate().skip(1) {
            new_trend = get_trend(prev, x);
            if current_trend != new_trend {
                ranges.push((start, idx - 1, current_trend));
                start = idx - 1;
            }
            current_trend = new_trend;

            prev = x;
        }

        let mut res = i64::MIN;
        ranges.push((start, nums.len() - 1, current_trend));

        for range_idx in 1..ranges.len() - 1 {
            let prev_range = &ranges[range_idx - 1];
            let curr_range = &ranges[range_idx];
            let next_range = &ranges[range_idx + 1];

            if prev_range.2 != TREND::INCREASING
                || curr_range.2 != TREND::DECREASING
                || next_range.2 != TREND::INCREASING
            {
                continue;
            }

            // println!("prev_range: {:?}", prev_range);
            let mut prev_max = i64::MIN;
            let mut sum = 0;
            for idx in (prev_range.0..prev_range.1).rev() {
                sum += nums[idx];
                prev_max = prev_max.max(sum);
            }
            // println!("prev max: {}", prev_max);

            // println!("curr_range: {:?}", curr_range);
            let curr_sum = (curr_range.0..=curr_range.1).map(|x| nums[x]).sum::<i64>();
            // println!("curr sum: {}", curr_sum);

            // println!("next_range: {:?}", next_range);
            let mut next_max = i64::MIN;
            let mut sum = 0;
            for idx in next_range.0 + 1..=next_range.1 {
                sum += nums[idx];
                next_max = next_max.max(sum);
            }
            // println!("next max: {}", next_max);

            res = res.max(prev_max + curr_sum + next_max);
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![0, -2, -1, -3, 0, 2, -1];
        let res = -4;
        assert_eq!(Solution::max_sum_trionic(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 4, 2, 7];
        let res = 14;
        assert_eq!(Solution::max_sum_trionic(nums), res);
    }
}
