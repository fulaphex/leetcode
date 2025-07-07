use std::cmp;

struct Solution {}

impl Solution {
    pub fn do_intervals_intersect(interval: &Vec<i32>, other_interval: &Vec<i32>) -> bool {
        assert_eq!(interval.len(), 2);
        assert_eq!(other_interval.len(), 2);
        let max_start = cmp::max(interval[0], other_interval[0]);
        let min_end = cmp::min(interval[1], other_interval[1]);
        return max_start <= min_end;
    }

    pub fn get_interval_intersection(interval: Vec<i32>, other_interval: Vec<i32>) -> Vec<i32> {
        assert!(Self::do_intervals_intersect(
            interval.as_ref(),
            other_interval.as_ref()
        ));
        let min_start = cmp::min(interval[0], other_interval[0]);
        let max_end = cmp::max(interval[1], other_interval[1]);
        let result = vec![min_start, max_end];
        return result;
    }

    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new_interval_mut = Some(new_interval);
        let mut res: Vec<Vec<i32>> = Vec::new();
        for interval in intervals {
            println!(
                "new_interval: {:?}, interval: {:?}",
                new_interval_mut.as_ref().unwrap(),
                interval
            );
            if new_interval_mut.is_none() {
                println!("new interval already processed");
                // new interval processed - just add the rest of original intervals to the result
                res.push(interval);
            } else {
                if Self::do_intervals_intersect(
                    interval.as_ref(),
                    new_interval_mut.as_ref().unwrap(),
                ) {
                    println!("intervals intersect");
                    new_interval_mut = Some(Self::get_interval_intersection(
                        interval,
                        new_interval_mut.unwrap(),
                    ));
                } else {
                    println!("intervals not intersect");
                    let new_interval_val = new_interval_mut.as_ref().unwrap();
                    if interval[0] < new_interval_val[0] {
                        println!("interval before new interval - adding interval");
                        // 1. new interval is before interval -> add new interval to result
                        res.push(interval);
                    } else {
                        println!("interval before new interval - adding new interval");
                        res.push(new_interval_val.to_vec());
                        res.push(interval);
                        new_interval_mut = None;
                        // 2. new interval is after interval -> add interval to result
                    }
                }
            }
        }
        if new_interval_mut.is_some() {
            res.push(new_interval_mut.unwrap());
        }
        return res;
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_simple() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 5], vec![6, 9]]);
    }

    #[test]
    fn test_spans_multiple() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![1, 2], vec![3, 10], vec![12, 16]]);
    }

    #[test]
    fn test_no_intervals() {
        let intervals = vec![];
        let new_interval = vec![4, 8];
        let result = Solution::insert(intervals, new_interval);
        assert_eq!(result, vec![vec![4, 8],]);
    }
}
