impl Solution {
    fn gcd(a: usize, b: usize) -> usize {
        if a < b {
            return Self::gcd(b, a);
        }
        if b == 0 {
            return a;
        }
        Self::gcd(b, a % b)
    }
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        let a = a as u8;
        let nums: Vec<u8> = s.as_bytes().iter().map(|x| x - b'0').collect();
        let rot_step = Self::gcd(nums.len(), b as usize);

        let add_count = 10;
        let mut res = nums.clone();
        let mut curr = vec![0; nums.len()];

        for rot in (0..nums.len()).step_by(rot_step) {
            let offset = rot;

            let (first, second) = (nums[offset], nums[(offset + 1) % nums.len()]);
            let (mut best_add_first, mut best_add_second, mut best_first, mut best_second) =
                (0, 0, first, second);
            for add in 0..add_count {
                let (new_first, new_second) = ((first + add * a) % 10, (second + add * a) % 10);
                if new_first < best_first {
                    best_first = new_first;
                    best_add_first = add;
                }
                if new_second < best_second {
                    best_second = new_second;
                    best_add_second = add;
                }
            }
            if b % 2 == 0 {
                best_add_first = 0;
            }

            for idx in 0..nums.len() {
                let new_idx = (idx + offset) % nums.len();
                curr[idx] = nums[new_idx];

                if idx % 2 == 0 {
                    curr[idx] += best_add_first * a;
                } else {
                    curr[idx] += best_add_second * a;
                }
                curr[idx] %= 10;
            }

            if curr < res {
                (res, curr) = (curr, res);
            }
        }

        String::from_iter(res.iter().map(|&x| (b'0' + x) as char))
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("5525");
        let (a, b) = (9, 2);
        let res = String::from("2050");

        assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
    }

    #[test]
    fn test2() {
        let s = String::from("74");
        let (a, b) = (5, 1);
        let res = String::from("24");

        assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
    }

    #[test]
    fn test3() {
        let s = String::from("0011");
        let (a, b) = (4, 2);
        let res = String::from("0011");

        assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
    }

    #[test]
    fn test4() {
        let s = String::from("5562438547");
        let (a, b) = (1, 3);
        let res = String::from("0014305132");

        assert_eq!(Solution::find_lex_smallest_string(s, a, b), res);
    }
}
