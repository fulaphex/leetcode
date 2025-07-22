use std::iter;

struct Solution {}

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<i32>) -> String {
        let mut letters = s.chars().collect::<Vec<char>>();
        let mut shift_sum: i32 = shifts.iter().map(|x| x % 26).sum();

        for (chr, shift) in iter::zip(&mut letters, shifts) {
            let mut letter_idx = *chr as i32 - 'a' as i32;
            letter_idx = (letter_idx + shift_sum) % 26;
            *chr = (('a' as i32 + letter_idx) as u8) as char;
            shift_sum = (shift_sum + 26 - (shift % 26)) % 26;
        }

        return String::from_iter(letters);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("abc");
        let shifts = vec![3, 5, 9];
        let res = String::from("rpl");
        assert_eq!(Solution::shifting_letters(s, shifts), res);
    }

    #[test]
    fn test2() {
        let s = String::from("aaa");
        let shifts = vec![1, 2, 3];
        let res = String::from("gfd");
        assert_eq!(Solution::shifting_letters(s, shifts), res);
    }
}
