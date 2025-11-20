impl Solution {
    fn num_higher_equal(arr: &[i32], x: i32) -> usize {
        let mut res = 0;
        if arr[arr.len() - 1] < x {
            return res;
        }

        for &el in arr.iter().rev().take(2) {
            if el >= x {
                res += 1;
            } else {
                break;
            }
        }
        res
    }

    pub fn intersection_size_two(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.is_empty() {
            return 0;
        }
        let mut intervals = intervals.iter().map(|v| (v[0], v[1])).collect::<Vec<_>>();
        intervals.sort_unstable_by_key(|x| (x.1, x.0));

        let mut required = vec![intervals[0].1 - 1, intervals[0].1];

        for &interval in &intervals {
            let to_add = 2 - Self::num_higher_equal(&required, interval.0);
            let end = interval.1;

            if to_add == 2 {
                required.push(end - 1);
                required.push(end);
            } else if to_add == 1 {
                let len = required.len();
                required[len - 1] = required[len - 1].min(end - 1);
                required.push(end);
            }
        }

        required.len() as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let intervals = [[1, 3], [3, 7], [8, 9]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 5;

        assert_eq!(Solution::intersection_size_two(intervals), res);
    }

    #[test]
    fn test2() {
        let intervals = [[1, 3], [1, 4], [2, 5], [3, 5]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 3;

        assert_eq!(Solution::intersection_size_two(intervals), res);
    }

    #[test]
    fn test3() {
        let intervals = [[1, 2], [2, 3], [2, 4], [4, 5]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 5;

        assert_eq!(Solution::intersection_size_two(intervals), res);
    }

    #[test]
    fn test4() {
        let intervals = [[1, 3], [3, 7], [5, 7], [7, 8]]
            .iter()
            .map(Vec::from)
            .collect::<Vec<_>>();
        let res = 5;

        assert_eq!(Solution::intersection_size_two(intervals), res);
    }
}
