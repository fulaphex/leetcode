struct Solution {}

impl Solution {
    pub fn _inner(arr: &[i32], k: i32, acc: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if k == 0 {
            res.push(acc.clone());
        }
        if arr.len() < (k as usize) {
            return;
        }
        for (idx, &el) in arr.iter().enumerate() {
            acc.push(el);
            Self::_inner(&arr[idx + 1..], k - 1, acc, res);
            acc.pop();
        }
        return;
    }
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut acc: Vec<i32> = vec![];
        let mut res: Vec<Vec<i32>> = vec![];
        Self::_inner(&(1..n + 1).collect::<Vec<_>>(), k, &mut acc, &mut res);
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let k = 2;
        let res: Vec<Vec<i32>> = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];
        assert_eq!(Solution::combine(n, k), res);
    }

    #[test]
    fn test2() {
        let n = 1;
        let k = 1;
        let res: Vec<Vec<i32>> = vec![vec![1]];
        assert_eq!(Solution::combine(n, k), res);
    }
}
