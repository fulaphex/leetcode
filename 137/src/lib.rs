impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut counts = vec![0; 32];
        for x in nums {
            for (byte_idx, &byte) in x.to_be_bytes().iter().enumerate() {
                let mut byte = byte;
                for i in 0..8 {
                    counts[byte_idx * 8 + i] += byte % 2;
                    counts[byte_idx * 8 + i] %= 3;
                    byte /= 2;
                }
            }
        }
        let mut bytes = [0_u8; 4];
        for (idx, c) in counts.iter().enumerate() {
            bytes[idx / 8] += c * (1 << (idx % 8));
        }
        return i32::from_be_bytes(bytes);
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 2, 3, 2];
        let res = 3;
        assert_eq!(Solution::single_number(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![0, -1, 0, -1, 0, -1, -99];
        let res = -99;
        assert_eq!(Solution::single_number(nums), res);
    }
}
