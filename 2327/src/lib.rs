use std::collections::VecDeque;

impl Solution {
    const MOD: i64 = 1_000_000_000 + 7;
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        // people that know but don't say
        let mut aware = VecDeque::new();
        aware.push_front(1_i64);
        for _ in 0..delay - 1 {
            aware.push_back(0);
        }
        // people that say but didn't forget
        let mut telling = VecDeque::new();
        for _ in 0..forget - delay {
            telling.push_back(0);
        }
        // total people who ever knew
        // sum of aware
        let mut aware_sum = 1;
        // sum of telling
        let mut telling_sum = 0;
        for _ in 0..n - 1 {
            println!("state: {:?} {:?}|", aware, telling);
            telling_sum -= telling.pop_back().unwrap();
            let last_aware = aware.pop_back().unwrap();
            aware_sum -= last_aware;
            telling_sum += last_aware;
            telling_sum %= Self::MOD;
            telling.push_front(last_aware);

            aware_sum += telling_sum;
            aware_sum %= Self::MOD;
            aware.push_front(telling_sum);
        }
        // println!("state: {:?} {:?}|", aware, telling);
        return ((aware_sum + telling_sum) % Self::MOD) as i32;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        // 6 2 4 5
        let (n, delay, forget) = (6, 2, 4);
        let res = 5;
        assert_eq!(Solution::people_aware_of_secret(n, delay, forget), res);
    }

    #[test]
    fn test2() {
        // 4 1 3 6
        let (n, delay, forget) = (4, 1, 3);
        let res = 6;
        assert_eq!(Solution::people_aware_of_secret(n, delay, forget), res);
    }
}
