use std::usize;

impl Solution {
    pub fn merge_stones(stones: Vec<i32>, k: i32) -> i32 {
        let (n, uk) = (stones.len(), k as usize);
        if (stones.len() - 1) % (uk - 1) > 0 {
            return -1;
        }
        let mut pref_sum = vec![0; stones.len() + 1];
        for (idx, &x) in stones.iter().enumerate() {
            pref_sum[idx + 1] = pref_sum[idx] + x;
        }
        let mut cache = vec![vec![vec![None; n + uk]; n + uk]; uk + n + 2];

        fn dp(
            start_pos: usize,
            end_pos: usize,
            piles: usize,
            pref_sum: &Vec<i32>,
            k: usize,
            // cache: &mut HashMap<(usize, usize, usize), Option<i32>>,
            cache: &mut Vec<Vec<Vec<Option<Option<i32>>>>>,
        ) -> Option<i32> {
            // check if value already computed
            let val = cache[start_pos][end_pos][piles];
            if val.is_some() {
                return val.unwrap();
            }

            // if there's one stone - return 0 if one pile else None
            if end_pos == start_pos {
                return if piles == 1 { Some(0) } else { None };
            }
            // if there's one pile - split into k piles paying the cost
            if piles == 1 {
                let mut res = dp(start_pos, end_pos, k, pref_sum, k, cache);
                if res.is_some() {
                    res = Some(res.unwrap() + pref_sum[end_pos + 1] - pref_sum[start_pos]);
                    cache[start_pos][end_pos][piles] = Some(res);
                    return res;
                } else {
                    cache[start_pos][end_pos][piles] = Some(res);
                    return res;
                }
            }
            // otherwise cost of k piles is the best split (1, k-1)
            let mut res = None;
            for split in start_pos..end_pos {
                let (sub1, sub2) = (
                    dp(start_pos, split, 1, pref_sum, k, cache),
                    dp(split + 1, end_pos, piles - 1, pref_sum, k, cache),
                );
                if sub1.is_some() && sub2.is_some() {
                    res = Some(res.unwrap_or(i32::MAX).min(sub1.unwrap() + sub2.unwrap()));
                }
            }
            cache[start_pos][end_pos][piles] = Some(res);

            return res;
        }

        return dp(0, stones.len() - 1, 1, &pref_sum, k as usize, &mut cache).unwrap_or(-1) as i32;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let stones = vec![3, 2, 4, 1];
        let k = 2;
        let res = 20;
        assert_eq!(Solution::merge_stones(stones, k), res);
    }

    #[test]
    fn test2() {
        let stones = vec![3, 2, 4, 1];
        let k = 3;
        let res = -1;
        assert_eq!(Solution::merge_stones(stones, k), res);
    }

    #[test]
    fn test3() {
        let stones = vec![
            5, 13, 19, 98, 46, 16, 9, 10, 29, 57, 6, 70, 55, 95, 94, 47, 3, 30, 42,
        ];
        // let stones = vec![3, 5, 1, 2, 6];
        // let k = 3;
        let k = 19;
        let res = 25;
        assert_eq!(Solution::merge_stones(stones, k), res);
    }
}
