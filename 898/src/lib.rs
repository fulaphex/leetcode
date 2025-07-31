use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut res: HashSet<i32> = HashSet::from_iter(arr.iter().cloned());
        let mut bit_location = vec![arr.len() + 100; 33];

        for (idx, num) in arr.iter().enumerate() {
            // println!("num: {}", num);

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
            // println!("bit location: {:?}", bit_location);

            let mut x: HashMap<usize, i32> = HashMap::new();
            // based on bit locations
            if bit_location[0] < arr.len() {
                x.insert(bit_location[0], 0);
            }

            for (bit_idx, location) in bit_location.iter().enumerate().skip(1) {
                if *location >= arr.len() {
                    continue;
                }
                let mut value = x.get(location).unwrap_or(&0).clone();
                value += 1 << (bit_idx - 1);
                x.insert(*location, value);
            }
            // println!("x: {:?}", x);
            // println!("res: {:?}", res);
            let mut keys: Vec<&usize> = x.keys().collect();
            // println!("keys: {:?}", keys);
            keys.sort_by_key(|&&x| usize::MAX - x);
            // println!("keys: {:?}", keys);
            let mut or_val = x.get(keys[0]).unwrap().clone();
            // println!("{:?}", or_val);
            for &key in keys.iter().skip(1) {
                let val = x.get(key).unwrap();
                or_val |= val;
                res.insert(or_val);
            }

            // println!();
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
