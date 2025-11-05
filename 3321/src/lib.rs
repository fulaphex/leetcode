use std::collections::BTreeSet;
use std::collections::HashMap;

impl Solution {
    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let k = k as usize;
        let x = x as usize;
        let nums: Vec<_> = nums.into_iter().map(|x| x as i64).collect();

        let mut freqs: HashMap<i64, i64> = HashMap::new();
        for &n in &nums[..k] {
            *freqs.entry(n).or_insert(0) += 1;
        }

        let mut large_freq_set = BTreeSet::new();
        let mut small_freq_set = BTreeSet::new();

        for (&el, &count) in &freqs {
            large_freq_set.insert((count, el));
        }
        while large_freq_set.len() > x {
            let el = large_freq_set.pop_first().unwrap();
            small_freq_set.insert(el);
        }

        let func = |freqs: &mut HashMap<_, _>,
                    el: i64,
                    delta_count: i64,
                    large_freq_set: &mut BTreeSet<(i64, i64)>,
                    small_freq_set: &mut BTreeSet<(i64, i64)>,
                    sum: &mut i64| {
            let el_count = freqs.entry(el).or_default();
            let key = (*el_count, el);

            if large_freq_set.get(&key).is_some() {
                large_freq_set.remove(&key);
                *sum -= *el_count * el;
            } else if small_freq_set.get(&key).is_some() {
                small_freq_set.remove(&key);
            }

            *el_count += delta_count;

            *sum += *el_count * el;
            if *el_count > 0 {
                large_freq_set.insert((*el_count, el));
            }

            // remove extra elements from large set
            while large_freq_set.len() > x {
                let el = large_freq_set.pop_first().unwrap();
                *sum -= el.0 * el.1;
                small_freq_set.insert(el);
            }

            // if large set is too small - move elements from small set
            while !small_freq_set.is_empty() && large_freq_set.len() < x {
                let small_elem = small_freq_set.pop_last().unwrap();
                *sum += small_elem.0 * small_elem.1;
                large_freq_set.insert(small_elem);
            }

            // if smallest from large is smaller than larger from small - flip
            while !small_freq_set.is_empty()
                && large_freq_set.first().unwrap() < small_freq_set.last().unwrap()
            {
                let large_elem = large_freq_set.pop_first().unwrap();
                let small_elem = small_freq_set.pop_last().unwrap();
                *sum -= large_elem.0 * large_elem.1;
                *sum += small_elem.0 * small_elem.1;
                large_freq_set.insert(small_elem);
                small_freq_set.insert(large_elem);
            }
        };

        let mut sum = large_freq_set
            .iter()
            .fold(0, |acc, (count, el)| acc + count * el);

        let mut res = vec![];
        res.push(sum);

        for i in k..nums.len() {
            let (new, rem) = (nums[i], nums[i - k]);
            func(
                &mut freqs,
                new,
                1,
                &mut large_freq_set,
                &mut small_freq_set,
                &mut sum,
            );
            func(
                &mut freqs,
                rem,
                -1,
                &mut large_freq_set,
                &mut small_freq_set,
                &mut sum,
            );
            res.push(sum);
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 2, 2, 3, 4, 2, 3];
        let k = 6;
        let x = 2;
        let res = vec![6, 10, 12];

        assert_eq!(Solution::find_x_sum(nums, k, x), res);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 8, 7, 8, 7, 5];
        let k = 2;
        let x = 2;
        let res = vec![11, 15, 15, 15, 12];

        assert_eq!(Solution::find_x_sum(nums, k, x), res);
    }

    #[test]
    fn test3() {
        let nums = vec![
            1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000,
        ];
        let k = 6;
        let x = 1;
        let res = vec![6000000000];

        assert_eq!(Solution::find_x_sum(nums, k, x), res);
    }
}
