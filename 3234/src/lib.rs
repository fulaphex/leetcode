impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let n = s.len();
        let mut res = 0;

        let mut one_count = Vec::with_capacity(n.isqrt() + 1);
        one_count.push(0);
        let mut last_idx = one_count.len() - 1;

        for (sidx, l) in s.chars().enumerate() {
            if l == '1' {
                one_count[last_idx] += 1;
            } else {
                one_count.push(0);
                last_idx = one_count.len() - 1;
            }

            let mut sum_count = 0;

            for (idx, count) in one_count.iter().rev().enumerate().take(sidx.isqrt() + 1) {
                sum_count += count;
                if idx == 0 {
                    res += count;
                } else {
                    let zero_squared = idx * idx;
                    let min_ones = (sum_count - count).max(zero_squared);

                    if min_ones <= sum_count {
                        res += sum_count - min_ones + 1;
                    }
                }
            }
        }

        res as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("00011");
        let res = 5;
        assert_eq!(Solution::number_of_substrings(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("101101");
        let res = 16;
        assert_eq!(Solution::number_of_substrings(s), res);
    }
}
