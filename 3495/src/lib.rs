impl Solution {
    fn solve(start: i32, end: i32) -> i64 {
        let (mut x, mut res) = (1, 0);
        while x <= end {
            res += (end - (start.max(x)) + 1) as i64;
            x = 4 * x;
        }
        return (res + 1) / 2;
    }

    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        return queries.into_iter().map(|q| Self::solve(q[0], q[1])).sum();
    }
}
/*
0 - 0
1,2,3 - 1
4,5,6,7,8,9,10,11,12,13,14,15 - 2
*/

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let queries = [[1, 2], [2, 4]].into_iter().map(|x| Vec::from(x)).collect();
        let res = 3;
        assert_eq!(Solution::min_operations(queries), res);
    }

    #[test]
    fn test2() {
        let queries = [[2, 6]].into_iter().map(|x| Vec::from(x)).collect();
        let res = 4;
        assert_eq!(Solution::min_operations(queries), res);
    }

    #[test]
    fn test3() {
        let queries = [[19, 23]].into_iter().map(|x| Vec::from(x)).collect();
        let res = 8;
        assert_eq!(Solution::min_operations(queries), res);
    }
}
