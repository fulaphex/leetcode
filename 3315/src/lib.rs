impl Solution {
    #[inline]
    fn get_min(x: i32) -> i32 {
        if x % 2 == 0 {
            return -1;
        }

        // println!("geting min for: {} / {:b}", x, x);
        let mut longest_ones = 1;
        while ((longest_ones) & x) == longest_ones {
            longest_ones = 2 * longest_ones + 1;
        }
        longest_ones /= 2;
        // println!("longest: {} / {:b}", longest_ones, longest_ones);
        x - longest_ones + longest_ones / 2
    }
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        nums.into_iter().map(Self::get_min).collect::<Vec<_>>()
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 3, 5, 7];
        let res = vec![-1, 1, 4, 3];
        assert_eq!(Solution::min_bitwise_array(nums), res);
    }
}
