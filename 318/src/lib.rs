impl Solution {
    fn get_word_repr(s: &String) -> u32 {
        let mut arr = vec![false; 26];
        for c in s.chars() {
            arr[c as usize - 'a' as usize] = true;
        }

        let mut x = 0;
        for (i, v) in arr.into_iter().enumerate() {
            if v {
                x += 1 << i;
            }
        }
        return x;
    }

    pub fn max_product(mut words: Vec<String>) -> i32 {
        words.sort_unstable_by_key(|x| usize::MAX - x.len());

        let mut word_repr = vec![0; words.len()];
        for (idx, word) in words.iter().enumerate() {
            word_repr[idx] = Self::get_word_repr(word);
        }

        let mut res = 0;
        for (idx, word) in words.iter().enumerate() {
            if word.len() * word.len() <= res {
                break;
            }
            for (idx2, word2) in words.iter().enumerate().skip(idx + 1) {
                if word_repr[idx] & word_repr[idx2] == 0 {
                    res = res.max(word.len() * word2.len());
                }
            }
        }

        return res as i32;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words: Vec<String> = ["abcw", "baz", "foo", "bar", "xtfn", "abcdef"]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let res = 16;
        assert_eq!(Solution::max_product(words), res);
    }

    #[test]
    fn tests() {
        let words: Vec<String> = ["a", "ab", "abc", "d", "cd", "bcd", "abcd"]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let res = 4;
        assert_eq!(Solution::max_product(words), res);
    }

    #[test]
    fn test3() {
        let words: Vec<String> = ["a", "aa", "aaa", "aaaa"]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let res = 0;
        assert_eq!(Solution::max_product(words), res);
    }
}
