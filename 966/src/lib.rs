use std::collections::{HashMap, HashSet};

impl Solution {
    fn normalise_capitalisation(word: &String) -> String {
        return String::from_iter(word.chars().map(|x| x.to_ascii_lowercase()));
    }
    fn normalise_vowels(word: &String) -> String {
        return String::from_iter(word.chars().map(|x| {
            if x == 'a'
                || x == 'e'
                || x == 'i'
                || x == 'o'
                || x == 'u'
                || x == 'A'
                || x == 'E'
                || x == 'I'
                || x == 'O'
                || x == 'U'
            {
                '.'
            } else {
                x
            }
        }));
    }

    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let word_set: HashSet<_> = wordlist.iter().cloned().collect();
        let mut low_to_word = HashMap::new();
        let mut novowel_to_word = HashMap::new();
        for w in &wordlist {
            let lower = Self::normalise_capitalisation(w);
            let no_vowel = Self::normalise_vowels(&lower);

            println!("{} {}", w, &lower);
            if !low_to_word.contains_key(&lower) {
                low_to_word.insert(lower, w);
            }

            // let no_vowel = Self::normalise_vowels(w);
            println!("{} {}", w, &no_vowel);
            if !novowel_to_word.contains_key(&no_vowel) {
                novowel_to_word.insert(no_vowel, w);
            }
        }

        println!("{:?}", low_to_word);
        println!("{:?}", novowel_to_word);

        let mut res: Vec<String> = vec![];

        for x in &queries {
            let lower = Self::normalise_capitalisation(x);
            let no_vowel = Self::normalise_vowels(&lower);

            println!("{} {} {}", x, lower, no_vowel);
            if word_set.contains(x) {
                res.push(x.clone());
            } else if low_to_word.contains_key(&lower) {
                res.push((*low_to_word.get(&lower).unwrap()).clone());
            } else if novowel_to_word.contains_key(&no_vowel) {
                res.push((*novowel_to_word.get(&no_vowel).unwrap()).clone());
            } else {
                res.push(String::new());
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
        let wordlist = ["KiTe", "kite", "hare", "Hare"]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let queries = [
            "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
        ]
        .into_iter()
        .map(|x| String::from(x))
        .collect();
        let res: Vec<_> = [
            "kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe",
        ]
        .into_iter()
        .map(|x| String::from(x))
        .collect();
        assert_eq!(Solution::spellchecker(wordlist, queries), res);
    }
}
