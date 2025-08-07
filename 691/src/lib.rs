use std::{collections, iter};

struct Solution {}

impl Solution {
    // fn parse(word: &String) -> Vec<i32> {
    //     let mut res = vec![0; 26];
    //     for l in word.chars() {
    //         res[l as usize - 'a' as usize] += 1;
    //     }
    //     return res;
    // }
    fn parse(word: &String) -> Vec<i8> {
        let mut res = vec![0; 26];
        for l in word.chars() {
            res[l as usize - 'a' as usize] += 1;
        }
        return res;
    }
    fn majorised_by(freq: &Vec<i8>, other: &Vec<i8>) -> bool {
        assert_eq!(freq.len(), other.len());
        return iter::zip(freq, other).all(|(&x, &y)| x <= y);
    }
    fn dist(freq: &Vec<i8>, other_freq: &Vec<i8>) -> i8 {
        // dist from freq to other_freq
        let mut res = 0;
        for (&x, &y) in iter::zip(freq, other_freq) {
            if x < y {
                res += y - x;
            }
        }
        return res;
    }
    fn add_state(state: &Vec<i8>, delta: &Vec<i8>) -> Vec<i8> {
        let mut res = state.clone();
        for (x, y) in iter::zip(res.iter_mut(), delta) {
            *x += y;
        }
        return res;
    }

    fn rem_state(state: &Vec<i8>, delta: &Vec<i8>) -> Vec<i8> {
        let mut res = state.clone();
        for (x, y) in iter::zip(res.iter_mut(), delta) {
            *x = 0.max(*x - *y);
            // *x -= y;
        }
        return res;
    }

    fn intersects(state: &Vec<i8>, delta: &Vec<i8>) -> bool {
        return iter::zip(state, delta).any(|(&x, &y)| x > 0 && y > 0);
    }

    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        // println!("{:?}", stickers);
        let stickers: Vec<String> = collections::HashSet::<String>::from_iter(stickers)
            .into_iter()
            .collect();
        // println!("{:?}", stickers);

        let mut sticker_letters = collections::HashSet::new();
        for sticker in &stickers {
            for l in sticker.chars() {
                sticker_letters.insert(l);
            }
        }
        for l in target.chars() {
            if sticker_letters.get(&l).is_none() {
                return -1;
            }
        }
        let mut to_remove: collections::HashSet<_> = collections::HashSet::new();
        let stickers_freqs: Vec<Vec<i8>> = stickers.iter().map(|x| Self::parse(x)).collect();
        let target_freq = Self::parse(&target);
        // println!("{:?}", stickers_freqs);
        for (idx, (s1, f1)) in iter::zip(&stickers, &stickers_freqs).enumerate() {
            for (idx2, f2) in stickers_freqs.iter().enumerate() {
                if idx == idx2 {
                    continue;
                }
                if Self::majorised_by(f1, f2) {
                    to_remove.insert(idx);
                    break;
                }
            }
        }
        // println!("{:?}", to_remove);
        // println!("{:?}", stickers);
        // println!("{:?}", stickers_freqs);
        // let stickers: Vec<String> = stickers
        //     .into_iter()
        //     .enumerate()
        //     .filter(|(x, _)| to_remove.get(x).is_none())
        //     .map(|(_, y)| y)
        //     .collect();
        let stickers_freqs: Vec<Vec<i8>> = stickers_freqs
            .into_iter()
            .enumerate()
            .filter(|(x, _)| to_remove.get(x).is_none())
            .map(|(_, y)| y)
            .collect();
        // println!("{:?}", stickers);
        // println!("{:?}", stickers_freqs);

        let init_state = vec![0; 26];
        // let mut que = collections::binary_heap::BinaryHeap::from([(0, init_state)]);
        let mut que = collections::binary_heap::BinaryHeap::from([(0, target_freq.clone())]);
        let mut visited = collections::HashSet::new();

        while que.len() > 0 {
            let (neg_dist, freq_val) = que.pop().unwrap();
            if visited.get(&freq_val).is_some() {
                continue;
            }
            visited.insert(freq_val.clone());
            // println!("{:?}", freq_val);
            if freq_val.iter().all(|&x| x == 0) {
                return -neg_dist;
            }
            // if Self::majorised_by(&target_freq, &freq_val) {
            //     return -neg_dist;
            // }
            for word_freq in &stickers_freqs {
                // if Self::intersects(&freq_val, &word_freq) {
                let new_state = Self::rem_state(&freq_val, word_freq);
                que.push((neg_dist - 1, new_state));
                // }
            }
        }
        assert!(false);

        return 999_999;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // let stickers = ["with", "example", "science", "without", "with"]
        // let stickers = ["with", "example", "science", "without"]
        let stickers = ["with", "example", "science"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let target = String::from("thehat");
        assert_eq!(Solution::min_stickers(stickers, target), 3);
    }

    #[test]
    fn test2() {
        let stickers = ["notice", "possible"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let target = String::from("basicbasic");
        assert_eq!(Solution::min_stickers(stickers, target), -1);
    }

    #[test]
    fn test3() {
        let stickers = [
            "heavy",
            "claim",
            "seven",
            "set",
            "had",
            "it",
            "dead",
            "jump",
            "design",
            "question",
            "sugar",
            "dress",
            "any",
            "special",
            "ground",
            "huge",
            "use",
            "busy",
            "prove",
            "there",
            "lone",
            "window",
            "trip",
            "also",
            "hot",
            "choose",
            "tie",
            "several",
            "be",
            "that",
            "corn",
            "after",
            "excite",
            "insect",
            "cat",
            "cook",
            "glad",
            "like",
            "wont",
            "gray",
            "especially",
            "level",
            "when",
            "cover",
            "ocean",
            "try",
            "clean",
            "property",
            "root",
            "wing",
        ]
        .iter()
        .map(|&x| String::from(x))
        .collect();
        let target = String::from("travelbell");
        assert_eq!(Solution::min_stickers(stickers, target), 4);
    }
}
