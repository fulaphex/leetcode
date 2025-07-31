use std::collections;

struct Solution {}

impl Solution {
    pub fn _inner_mem(
        sticks: &[i32],
        s1: i32,
        s2: i32,
        s3: i32,
        s4: i32,
        target: i32,
        mem: &mut collections::HashMap<(usize, i32, i32, i32, i32), bool>,
    ) -> bool {
        // cutting short - over the limit already
        if s1.max(s2).max(s3).max(s4) > target {
            return false;
        }

        // end condition
        if sticks.len() == 0 {
            return s1 == s2 && s1 == s3 && s1 == s4;
        }

        // checking if result is cached
        let key = (sticks.len(), s1, s2, s3, s4);
        if mem.contains_key(&key) {
            return *mem.get(&key).unwrap();
        }

        // adding to the first side
        let (ns1, ns2, ns3, ns4) = (s1 + sticks[0], s2, s3, s4);
        if Self::_inner_mem(&sticks[1..], ns1, ns2, ns3, ns4, target, mem) {
            return true;
        }

        // adding to the second side
        let (ns1, ns2, ns3, ns4): (i32, i32, i32, i32);
        let ns = s2 + sticks[0];
        if ns > s1 {
            (ns1, ns2, ns3, ns4) = (ns, s1, s3, s4);
        } else {
            (ns1, ns2, ns3, ns4) = (s1, ns, s3, s4);
        }
        if Self::_inner_mem(&sticks[1..], ns1, ns2, ns3, ns4, target, mem) {
            mem.insert(key, true);
            return true;
        }

        // adding to the third side
        let (ns1, ns2, ns3, ns4): (i32, i32, i32, i32);
        let ns = s3 + sticks[0];
        if ns > s1 {
            (ns1, ns2, ns3, ns4) = (ns, s2, s1, s4);
        } else if ns > s2 {
            (ns1, ns2, ns3, ns4) = (s1, ns, s2, s4);
        } else {
            (ns1, ns2, ns3, ns4) = (s1, s2, ns, s4);
        }
        if Self::_inner_mem(&sticks[1..], ns1, ns2, ns3, ns4, target, mem) {
            mem.insert(key, true);
            return true;
        }

        // adding to the fourth side
        let (ns1, ns2, ns3, ns4): (i32, i32, i32, i32);
        let ns = s4 + sticks[0];
        if ns > s1 {
            (ns1, ns2, ns3, ns4) = (ns, s2, s3, s1);
        } else if ns > s2 {
            (ns1, ns2, ns3, ns4) = (s1, ns, s3, s2);
        } else if ns > s3 {
            (ns1, ns2, ns3, ns4) = (s1, s2, ns, s3);
        } else {
            (ns1, ns2, ns3, ns4) = (s1, s2, s3, ns);
        }
        if Self::_inner_mem(&sticks[1..], ns1, ns2, ns3, ns4, target, mem) {
            mem.insert(key, true);
            return true;
        }
        mem.insert(key, false);
        return false;
    }

    pub fn makesquare(mut matchsticks: Vec<i32>) -> bool {
        matchsticks.sort_by_key(|x| -x);

        let matchstick_sum: i32 = matchsticks.iter().sum();
        if matchstick_sum % 4 != 0 {
            return false;
        }
        let side_length = matchstick_sum / 4;
        let mut mem = collections::HashMap::new();
        return Self::_inner_mem(&matchsticks, 0, 0, 0, 0, side_length, &mut mem);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let sticks = vec![1, 1, 2, 2, 2];
        let res = true;
        assert_eq!(Solution::makesquare(sticks), res);
    }

    #[test]
    fn test2() {
        let sticks = vec![3, 3, 3, 3, 4];
        let res = false;
        assert_eq!(Solution::makesquare(sticks), res);
    }

    #[test]
    fn test3() {
        let sticks = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 102];
        let res = false;
        assert_eq!(Solution::makesquare(sticks), res);
    }

    #[test]
    fn test4() {
        let sticks = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 70, 70, 101, 102, 103];
        let res = false;
        assert_eq!(Solution::makesquare(sticks), res);
    }

    #[test]
    fn test5() {
        let sticks = vec![
            5969561, 8742425, 2513572, 3352059, 9084275, 2194427, 1017540, 2324577, 6810719,
            8936380, 7868365, 2755770, 9954463, 9912280, 4713511,
        ];
        let res = false;
        assert_eq!(Solution::makesquare(sticks), res);
    }
}
