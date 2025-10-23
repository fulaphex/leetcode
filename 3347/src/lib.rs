use std::collections::{HashMap, VecDeque};

impl Solution {
    pub fn max_frequency(nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        let mut freqs = HashMap::new();
        for &n in &nums {
            *freqs.entry(n).or_insert(0) += 1;
        }
        let mut keys: Vec<i32> = freqs.keys().cloned().collect();
        keys.sort();
        println!("keys: {:?}", keys);

        let mut key_que = VecDeque::from_iter(keys.iter().cloned());
        let mut res = 0;
        // 1. iterate the windows of max width 2*k. num_operations elements can be changed to window_start + k
        //   - res = res.max(window.len().min(num_operations))
        let mut window = VecDeque::new();
        let mut window_count = 0;
        // let first = key_que.pop_front().unwrap();
        // let mut window = VecDeque::from([first]);
        // let mut window_count = freqs.get(&first).unwrap();
        while !key_que.is_empty() {
            let last = key_que.pop_front().unwrap();
            let last_count = freqs.get(&last).unwrap();
            window.push_back(last);
            window_count += last_count;
            while window.front().unwrap() + 2 * k < last {
                let el = window.pop_front().unwrap();
                window_count -= freqs.get(&el).unwrap();
            }
            res = res.max(window_count.min(num_operations));
            println!("{:?} {:?}", window, window_count);
        }

        // 2. iterate the windows [-k,k] around each element - num_operation elements are changed to the current element
        //   - res = res.max(freq[curr] + window.len().min(num_operations)  )
        println!("windows around element: ");
        let mut key_que = VecDeque::from_iter(keys.iter().cloned());

        let (mut before, mut after) = (VecDeque::new(), VecDeque::new());
        let (mut before_count, mut after_count) = (0, 0);
        for curr in keys {
            // add values to after
            while !key_que.is_empty() {
                // println!("while after");
                let &val = key_que.front().unwrap();
                if val <= (curr + k) {
                    key_que.pop_front();
                    if val > curr {
                        after.push_back(val);
                    }
                } else {
                    break;
                }
            }

            println!("after: {:?}", after);

            // get frequency of current
            let curr_freq = freqs.get(&curr).unwrap();
            while !before.is_empty() {
                // println!("while before");
                let &val = before.front().unwrap();
                if val < (curr - k) {
                    before.pop_front();
                } else {
                    break;
                }
            }
            println!("before: {:?}", before);

            //
            println!("curr: {}", curr);

            // END: move curr to before
            println!();
            before.push_back(curr);
        }
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums = vec![1, 4, 5];
        let k = 1;
        let num_operations = 2;
        let res = 2;

        assert_eq!(Solution::max_frequency(nums, k, num_operations), res);
    }
}
