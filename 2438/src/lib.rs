struct Solution {}

impl Solution {
    fn parse(n: i32) -> Vec<i32> {
        let mut res = vec![];
        let mut x = n;
        let mut idx = 0;
        while x > 0 {
            if x % 2 != 0 {
                res.push(idx);
            }
            idx += 1;
            x /= 2;
        }
        return res;
    }

    fn pow(n: i32) -> i32 {
        let modulo = 1_000_000_007;
        if n == 0 {
            return 1;
        }
        let x = Self::pow(n / 2) as i64;
        if n % 2 == 1 {
            return ((2 * x * x) % modulo) as i32;
        } else {
            return ((x * x) % modulo) as i32;
        }
    }

    pub fn product_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let arr = Self::parse(n);
        let mut res = vec![];
        let mut pref_sum = vec![0; arr.len() + 1];

        for (idx, x) in arr.iter().enumerate() {
            pref_sum[idx + 1] = x + pref_sum[idx];
        }
        let pows: Vec<_> = queries
            .iter()
            .map(|q| pref_sum[q[1] as usize + 1] - pref_sum[q[0] as usize])
            .enumerate()
            .collect();
        println!("{:?}", pows);

        for query in queries {
            let (start, end) = (query[0] as usize, query[1] as usize + 1);
            res.push(Self::pow(pref_sum[end] - pref_sum[start]));
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 15;
        let queries = [[0, 1], [2, 2], [0, 3]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = vec![2, 4, 64];
        assert_eq!(Solution::product_queries(n, queries), res);
    }

    #[test]
    fn test2() {
        let n = 2;
        let queries = [[0, 0]].iter().map(|x| Vec::from(x)).collect();
        let res = vec![2];
        assert_eq!(Solution::product_queries(n, queries), res);
    }
}
