impl Solution {
    pub fn max_free_time(
        event_time: i32,
        k: i32,
        mut start_time: Vec<i32>,
        mut end_time: Vec<i32>,
    ) -> i32 {
        let k = k as usize;
        start_time.push(event_time);
        end_time.push(event_time);

        let mut zip_iter = std::iter::zip(&start_time, &end_time);

        let mut running_sum = (&mut zip_iter).take(k).map(|(s, e)| e - s).sum::<i32>();

        let mut range_start = 0;
        let mut res = 0;

        for (idx, (start, end)) in zip_iter.enumerate() {
            res = res.max(start - range_start - running_sum);

            running_sum += end - start;

            let (front_start, front_end) = (start_time[idx], end_time[idx]);
            running_sum -= front_end - front_start;
            range_start = front_end;
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(5, 1, vec![1,3], vec![2,5], 2)]
    #[case(10, 1, vec![0,2,9], vec![1,4,10], 6)]
    #[case(5, 2, vec![0,1,2,3,4], vec![1,2,3,4,5], 0)]
    fn test(
        #[case] event_time: i32,
        #[case] k: i32,
        #[case] start_time: Vec<i32>,
        #[case] end_time: Vec<i32>,
        #[case] res: i32,
    ) {
        assert_eq!(
            Solution::max_free_time(event_time, k, start_time, end_time),
            res
        );
    }
}
