impl Solution {
    fn letter_counts(s: &String) -> Vec<usize> {
        let mut res = vec![0; 26];
        for c in s.chars() {
            res[(c as u8 - 'a' as u8) as usize] += 1;
        }
        return res;
    }
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut res = vec![words[0].clone()];
        let mut prev_repr = Self::letter_counts(&words[0]);
        println!("{:?}", prev_repr);
        for w in words.into_iter().skip(1) {
            let repr = Self::letter_counts(&w);
            if repr != prev_repr {
                res.push(w);
                prev_repr = repr;
            }
        }
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words = ["abba", "baba", "bbaa", "cd", "cd"]
            .into_iter()
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        let res = ["abba", "cd"]
            .into_iter()
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        assert_eq!(Solution::remove_anagrams(words), res);
    }

    #[test]
    fn test2() {
        let words = ["a", "b", "c", "d", "e"]
            .into_iter()
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        let res = ["a", "b", "c", "d", "e"]
            .into_iter()
            .map(|x| String::from(x))
            .collect::<Vec<_>>();
        assert_eq!(Solution::remove_anagrams(words), res);
    }
}
