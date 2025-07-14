use std::{cmp, iter};
struct Solution {}

impl Solution {
    pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        let mut meetings: Vec<(i32, i32)> = vec![(-1, 0)];
        for (start, end) in iter::zip(start_time, end_time) {
            meetings.push((start, end));
        }
        meetings.push((event_time, event_time + 1));
        println!("{:?}", meetings);
        println!("meeting count: {}", meetings.len());
        meetings.sort();
        let mut gaps: Vec<i32> = vec![];
        for window in meetings.windows(2) {
            let meeting = window[0];
            let next = window[1];
            println!("meeting: {:?}, next meeting: {:?}", meeting, next);
            gaps.push(next.0 - meeting.1);
        }
        println!("gaps: {:?}", gaps);

        let mut res = 0;
        println!();

        let mut gaps_pref_max = vec![0; gaps.len() + 1];
        println!("creating prefix max array for: {:?}", gaps);
        // gaps_pref_max[idx] = max of all gaps before idx
        for (idx, &gap) in gaps.iter().enumerate() {
            gaps_pref_max[idx + 1] = cmp::max(gaps_pref_max[idx], gap);
        }
        println!("gaps_pref: {:?}", gaps_pref_max);
        println!();

        let mut gaps_suff_max = vec![0; gaps.len() + 1];
        println!("creating suffix max array for: {:?}", gaps);
        // gaps_suff_max[idx] = max of all gaps after idx
        for (idx, &gap) in gaps.iter().enumerate().rev() {
            gaps_suff_max[idx] = cmp::max(gaps_suff_max[idx + 1], gap);
        }
        println!("gaps_suff: {:?}", gaps_suff_max);
        println!();

        for (idx, window) in meetings.windows(3).enumerate() {
            let prev = window[0];
            let meeting = window[1];
            let next = window[2];
            println!(
                "prev: {:?}, meeting: {:?}, next meeting: {:?}",
                prev, meeting, next
            );
            let meeting_duration = meeting.1 - meeting.0;
            // we can move the meeting to either end of the current interval
            res = cmp::max(res, next.0 - prev.1 - meeting_duration);

            if gaps_pref_max[idx] >= meeting_duration {
                println!(
                    "found gap in prefix (max: {}) for the meeting {:?} (duration: {})",
                    gaps_pref_max[idx], meeting, meeting_duration
                );
                res = cmp::max(res, next.0 - prev.1);
            }
            if gaps_suff_max[idx + 2] >= meeting_duration {
                println!(
                    "found gap in suffix (max: {}) for the meeting {:?} (duration: {})",
                    gaps_suff_max[idx], meeting, meeting_duration
                );
                res = cmp::max(res, next.0 - prev.1);
            }
            println!("res: {}", res);
        }

        return res;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_1() {
        let event_time = 5;
        let start_time = vec![1, 3];
        let end_time = vec![2, 5];
        let result = 2;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            result
        );
    }

    #[test]
    fn test_2() {
        let event_time = 10;
        let start_time = vec![0, 7, 9];
        let end_time = vec![1, 8, 10];
        let result = 7;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            result
        );
    }

    #[test]
    fn test_3() {
        let event_time = 10;
        let start_time = vec![0, 3, 7, 9];
        let end_time = vec![1, 4, 8, 10];
        let result = 6;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            result
        );
    }

    #[test]
    fn test_4() {
        let event_time = 5;
        let start_time = vec![0, 1, 2, 3, 4];
        let end_time = vec![1, 2, 3, 4, 5];
        let result = 0;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            result
        );
    }

    #[test]
    fn test_5() {
        let event_time = 41;
        let start_time = vec![17, 24];
        let end_time = vec![19, 25];
        let result = 24;
        assert_eq!(
            Solution::max_free_time(event_time, start_time, end_time),
            result
        );
    }
}
