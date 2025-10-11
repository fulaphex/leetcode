use std::collections::VecDeque;

impl Solution {
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.sort();
        let mut power_with_counts = vec![(power[0] as i64, 0)];
        for p in power {
            let p64 = p as i64;
            let last_idx = power_with_counts.len() - 1;
            let (lp, count) = power_with_counts[last_idx];
            if p64 == lp {
                power_with_counts[last_idx] = (lp, count + 1);
            } else {
                power_with_counts.push((p64, 1));
            }
        }
        let mut que = VecDeque::from([(-100, 0)]);
        for &(curr_power, count) in power_with_counts.iter() {
            // remove elements from the front of the queue that
            // - we only want the biggest power smaller than p-2
            while que.len() > 1 {
                let &(_, res) = que.front().unwrap();
                let &(power, _) = que.get(1).unwrap();

                if power < curr_power - 2 {
                    que.pop_front();
                    let front = que.front_mut().unwrap();
                    front.1 = front.1.max(res);
                } else {
                    break;
                }
            }
            let (first_power, first_result) = que.front().unwrap();
            // assert!(first_power + 2 < curr_power);

            let curr_res = first_result + curr_power * count;
            que.push_back((curr_power, curr_res));
        }
        return que.iter().map(|&(_, x)| x).max().unwrap();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let energy = vec![1, 1, 3, 4];
        let res = 6;
        assert_eq!(Solution::maximum_total_damage(energy), res);
    }

    #[test]
    fn test2() {
        let energy = vec![5, 9, 2, 10, 2, 7, 10, 9, 3, 8];
        // let energy = vec![7, 1, 6, 6];
        let res = 31;
        // let res = 13;
        assert_eq!(Solution::maximum_total_damage(energy), res);
    }

    #[test]
    fn test3() {
        let energy = vec![7, 1, 6, 6];
        let res = 13;
        assert_eq!(Solution::maximum_total_damage(energy), res);
    }
}
