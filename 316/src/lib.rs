#[inline]
fn az_pos(c: char) -> usize {
    return (c as u8 - 'a' as u8) as usize;
}

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let last_char_pos = s
            .chars()
            .enumerate()
            .fold(vec![0; 26], |mut arr, (idx, char)| {
                arr[az_pos(char)] = idx;
                arr
            });

        let mut letter_used = vec![false; 26];
        let mut stack = vec![];

        for (idx, char) in s.chars().enumerate() {
            if letter_used[az_pos(char)] {
                continue;
            }
            while !stack.is_empty() {
                let last_letter = stack[stack.len() - 1];
                if char < last_letter && last_char_pos[az_pos(last_letter)] > idx {
                    letter_used[az_pos(last_letter)] = false;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(char);
            letter_used[az_pos(char)] = true;
        }

        return String::from_iter(stack.iter());
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("bcabc");
        let res = String::from("abc");
        assert_eq!(Solution::remove_duplicate_letters(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("cbacdcbc");
        let res = String::from("acdb");
        assert_eq!(Solution::remove_duplicate_letters(s), res);
    }
}
