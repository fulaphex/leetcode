impl Solution {
    fn into_bits(mut x: i32) -> Vec<bool> {
        let mut res = vec![false; 8];
        for b in res.iter_mut().rev() {
            *b = x % 2 == 1;
            x /= 2;
        }
        assert_eq!(x, 0);
        return res;
    }
    fn _valid(data: &[i32]) -> bool {
        if data.len() == 0 {
            return true;
        }
        let bits = Self::into_bits(data[0]);

        if bits[0] == false {
            return Self::_valid(&data[1..]);
        } else {
            let mut length = 0;
            for &x in bits.iter().take(4) {
                if x == true {
                    length += 1;
                } else {
                    break;
                }
            }

            if bits[length] == true || length == 1 || data.len() < length {
                return false;
            }

            for &x in data.iter().take(length).skip(1) {
                let bits = Self::into_bits(x);
                if !(bits[0] == true && bits[1] == false) {
                    return false;
                }
            }

            return Self::_valid(&data[length..]);
        }
    }
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        return Self::_valid(data.as_slice());
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let data = vec![197, 130, 1];
        let res = true;
        assert_eq!(Solution::valid_utf8(data), res);
    }

    #[test]
    fn test2() {
        let data = vec![235, 140, 4];
        let res = false;
        assert_eq!(Solution::valid_utf8(data), res);
    }

    #[test]
    fn test3() {
        let data = vec![248, 130, 130, 130];
        let res = false;
        assert_eq!(Solution::valid_utf8(data), res);
    }
}
