// solution using binary search
impl Solution {
    fn search(potions: &[i32], target: i64) -> usize {
        if target <= potions[0] as i64 {
            return 0;
        }
        if target > potions[potions.len() - 1] as i64 {
            return potions.len();
        }

        let mid_idx = potions.len() / 2;
        let mid = potions[mid_idx] as i64;
        if mid >= target {
            return Self::search(&potions[..mid_idx], target);
        } else {
            return mid_idx + Self::search(&potions[mid_idx..], target);
        }
    }
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort();
        let mut res = vec![];

        for s in spells {
            let s64 = s as i64;
            let potion_tgt = (success + s64 - 1) / s64;
            let idx = Self::search(potions.as_slice(), potion_tgt);

            res.push((potions.len() - idx) as i32);
        }
        return res;
    }
}

// sorting both arrays then traversing them once
// impl Solution {
//     pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
//         potions.sort_unstable();
//         let mut sorted_indexed_spells: Vec<_> =
//             spells.iter().enumerate().map(|(idx, x)| (x, idx)).collect();
//         sorted_indexed_spells.sort_unstable();

//         let mut idx = potions.len();
//         let mut res = vec![0; spells.len()];

//         for (&s, spell_idx) in sorted_indexed_spells {
//             let potion_target = (success + s as i64 - 1) / s as i64;
//             while idx != 0 && potions[idx - 1] as i64 >= potion_target {
//                 idx -= 1;
//             }
//             res[spell_idx] = (potions.len() - idx) as i32;
//         }
//         return res;
//     }
// }

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let spells = vec![5, 1, 3];
        let potions = vec![1, 2, 3, 4, 5];
        let success = 7;
        let res = vec![4, 0, 3];
        assert_eq!(Solution::successful_pairs(spells, potions, success), res);
    }

    #[test]
    fn test2() {
        let spells = vec![3, 1, 2];
        let potions = vec![8, 5, 8];
        let success = 16;
        let res = vec![2, 0, 2];
        assert_eq!(Solution::successful_pairs(spells, potions, success), res);
    }
}
