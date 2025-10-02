impl Solution {
    fn solve(
        curr: i32,
        target_length: usize,
        target: i32,
        sum: i32,
        acc: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
    ) {
        println!("curr: {}; acc: {:?}; sum: {}", curr, acc, sum);
        if sum > target {
            return;
        }
        if acc.len() == target_length {
            if sum == target {
                res.push(acc.clone());
            }
        } else {
            if curr >= 10 {
                return;
            }
            assert!(acc.len() < target_length);
            acc.push(curr);
            Self::solve(curr + 1, target_length, target, sum + curr, acc, res);
            acc.pop();
            Self::solve(curr + 1, target_length, target, sum, acc, res);
        }
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::solve(1, k as usize, n, 0, &mut vec![], &mut res);
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (k, n) = (3, 7);
        let res: Vec<_> = [[1, 2, 4]].into_iter().map(|x| Vec::from(x)).collect();
        assert_eq!(Solution::combination_sum3(k, n), res);
    }

    #[test]
    fn test2() {
        let (k, n) = (3, 9);
        let res: Vec<_> = [[1, 2, 6], [1, 3, 5], [2, 3, 4]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        assert_eq!(Solution::combination_sum3(k, n), res);
    }

    #[test]
    fn test3() {
        let (k, n) = (4, 1);
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combination_sum3(k, n), res);
    }

    #[test]
    fn test4() {
        let (k, n) = (9, 45);
        let res: Vec<Vec<i32>> = vec![];
        assert_eq!(Solution::combination_sum3(k, n), res);
    }
}
