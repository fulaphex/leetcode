impl Solution {
    fn check_fast(n: i64, runtime: i64, batteries: &[i64]) -> bool {
        batteries.iter().map(|&bat| bat.min(runtime)).sum::<i64>() >= n as i64 * runtime
    }

    pub fn max_run_time(n: i32, batteries: Vec<i32>) -> i64 {
        let batteries: Vec<i64> = batteries.into_iter().map(|x| x as i64).collect();
        let n = n as i64;

        let (mut start, mut end) = (0, batteries.iter().sum::<i64>() / n + 1);

        while (end - start) > 1 {
            let mid = (start + end + 1) / 2;
            if Self::check_fast(n, mid, &batteries) {
                start = mid;
            } else {
                end = mid;
            }
        }

        start
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 2;
        let batteries = vec![3, 3, 3];
        let res = 4;
        assert_eq!(Solution::max_run_time(n, batteries), res);
    }

    #[test]
    fn test2() {
        let n = 2;
        let batteries = vec![1, 1, 1, 1];
        let res = 2;
        assert_eq!(Solution::max_run_time(n, batteries), res);
    }
}
