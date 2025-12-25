impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        happiness.sort_unstable();
        happiness
            .iter()
            .rev()
            .enumerate()
            .take(k as usize)
            .fold(0, |acc, (idx, &x)| acc + (x as i64 - idx as i64).max(0))
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let happiness = vec![1, 2, 3];
        let k = 2;
        let res = 4;
        assert_eq!(Solution::maximum_happiness_sum(happiness, k), res);
    }
}
