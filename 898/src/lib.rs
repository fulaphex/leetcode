use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut res: HashSet<i32> = HashSet::from_iter(arr.iter().cloned());
        let mut bit_location = vec![arr.len() + 100; 33];

        for (idx, num) in arr.iter().enumerate() {
            // update the bit locations
            let mut bit_idx = 1;
            if *num == 0 {
                bit_location[0] = idx;
            } else {
                let mut numm = *num;
                while numm > 0 {
                    if numm % 2 == 1 {
                        bit_location[bit_idx] = idx;
                    }
                    numm /= 2;
                    bit_idx += 1;
                }
            }

            let mut x: HashSet<usize> = HashSet::new();
            // based on bit locations
            for location in &bit_location {
                if *location >= arr.len() {
                    continue;
                }
                x.insert(*location);
            }
            let mut keys: Vec<&usize> = x.iter().collect();
            keys.sort_by_key(|&&x| usize::MAX - x);
            let mut or_val = arr[*keys[0]];
            for &key in keys.iter().skip(1) {
                let val = arr[*key];
                or_val |= val;
                res.insert(or_val);
            }
        }
        return res.len() as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let arr = vec![0];
        let res = 1;
        assert_eq!(Solution::subarray_bitwise_o_rs(arr), res);
    }

    #[test]
    fn test2() {
        let arr = vec![1, 1, 2];
        let res = 3;
        assert_eq!(Solution::subarray_bitwise_o_rs(arr), res);
    }

    #[test]
    fn test3() {
        let arr = vec![1, 2, 4];
        let res = 6;
        assert_eq!(Solution::subarray_bitwise_o_rs(arr), res);
    }
}
