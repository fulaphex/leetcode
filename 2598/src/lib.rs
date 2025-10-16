impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut mods = vec![0; value as usize];

        for &n in &nums {
            let x = (-n / value + 1).max(0) * value + n;
            mods[(x % value) as usize] += 1;
        }

        let (remainder, count) = mods
            .iter()
            .enumerate()
            .min_by_key(|&(idx, &val)| (val, idx))
            .unwrap();
        value * count + remainder as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, -10, 7, 13, 6, 8];
        let nums = vec![1, -10, -3, 13, 6, 8];
        let value = 5;
        let res = 4;
        assert_eq!(Solution::find_smallest_integer(nums, value), res);
    }
}
