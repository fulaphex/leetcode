impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        let is_vowel = {
            let mut x = vec![false; 30];
            x[(b'a' - b'a') as usize] = true;
            x[(b'e' - b'a') as usize] = true;
            x[(b'i' - b'a') as usize] = true;
            x[(b'o' - b'a') as usize] = true;
            x[(b'u' - b'a') as usize] = true;
            x
        };

        let mut counts = vec![0; 30];
        for c in s.bytes() {
            counts[(c - b'a') as usize] += 1;
        }
        let (mut consonant_count, mut vowel_count) = (0, 0);
        for (idx, &count) in counts.iter().enumerate() {
            if is_vowel[idx] {
                vowel_count = vowel_count.max(count);
            } else {
                consonant_count = consonant_count.max(count);
            }
        }

        return vowel_count + consonant_count;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("successes");
        let res = 6;
        assert_eq!(Solution::max_freq_sum(s), res);
    }
}
