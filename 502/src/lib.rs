use std::{collections::BinaryHeap, iter::zip};

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let (mut cap, mut taken) = (w, 0);
        let mut profit_cap: Vec<(i32, i32)> = zip(profits, capital).collect();
        profit_cap.sort_unstable_by_key(|&(_p, c)| c);

        let mut available = BinaryHeap::new();
        for (p, c) in profit_cap {
            while c > cap {
                if available.is_empty() || taken == k {
                    // if k projects are already taken - we can return the result
                    // if there are no available projects to be taken but we still cannot afford
                    // the current project - next ones will only be more expensive
                    return cap;
                } else {
                    cap += available.pop().unwrap();
                    taken += 1;
                }
            }
            available.push(p);
        }
        while !available.is_empty() && taken < k {
            cap += available.pop().unwrap();
            taken += 1;
        }

        return cap;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (k, w) = (2, 0);
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 1];
        let res = 4;
        assert_eq!(
            Solution::find_maximized_capital(k, w, profits, capital),
            res
        );
    }

    #[test]
    fn test2() {
        let (k, w) = (3, 0);
        let profits = vec![1, 2, 3];
        let capital = vec![0, 1, 2];
        let res = 6;
        assert_eq!(
            Solution::find_maximized_capital(k, w, profits, capital),
            res
        );
    }
}
