impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        /* O(n) memory
        let mut arr = vec![false; nums.len()];
        let mut res = vec![];
        for x in nums {
            if arr[x as usize] {
                res.push(x);
            }
            arr[x as usize] = true;
        }
        res
        */
        // using xor magic
        let n = nums.len() - 2;
        let nums_xor = nums.iter().fold(0, |x, &y| x ^ y);
        let xy_xor = nums_xor ^ ((0..n as i32).reduce(|x, y| x ^ y)).unwrap();
        let lowest_bit = xy_xor & -xy_xor;

        let (mut x, mut y) = (0, 0);
        for el in nums.into_iter().chain(0..n as i32) {
            if (el & lowest_bit) > 0 {
                x ^= el;
            } else {
                y ^= el;
            }
        }

        vec![x, y]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![0, 1, 1, 0];
        let res = vec![0, 1];
        let mut out = Solution::get_sneaky_numbers(nums);
        out.sort_unstable();
        assert_eq!(out, res);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 3, 2, 1, 3, 2];
        let res = vec![2, 3];
        let mut out = Solution::get_sneaky_numbers(nums);
        out.sort_unstable();
        assert_eq!(out, res);
    }

    #[test]
    fn test3() {
        let nums = vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2];
        let res = vec![4, 5];
        let mut out = Solution::get_sneaky_numbers(nums);
        out.sort_unstable();
        assert_eq!(out, res);
    }
}
