impl Solution {
    fn f(n: usize, mem: &mut Vec<i32>) -> i32 {
        if mem[n] == -1 {
            let mut res = 0;
            for x in 1..=n {
                let delta = Self::f(x - 1, mem) * Self::f(n - x, mem);
                res += delta;
            }
            mem[n] = res;
        }
        mem[n]
    }
    pub fn num_trees(n: i32) -> i32 {
        let mut mem = vec![-1; 20];
        mem[0] = 1;
        Self::f(n as usize, &mut mem)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 3;
        let res = 5;
        assert_eq!(Solution::num_trees(n), res);
    }

    #[test]
    fn test3() {
        let n = 19;
        let res = 1767263190;
        assert_eq!(Solution::num_trees(n), res);
    }

    #[test]
    fn test2() {
        let n = 1;
        let res = 1;
        assert_eq!(Solution::num_trees(n), res);
    }
}
