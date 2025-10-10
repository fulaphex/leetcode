impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let uk = k as usize;
        let (mut res, mut sums) = (i32::MIN, vec![0; uk]);
        for (idx, &e) in energy.iter().enumerate().rev() {
            sums[idx % uk] += e;
            res = res.max(sums[idx % uk]);
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
        let energy = vec![5, 2, -10, -5, 1];
        let k = 3;
        let res = 3;
        assert_eq!(Solution::maximum_energy(energy, k), res);
    }

    #[test]
    fn test2() {
        let energy = vec![-2, -3, -1];
        let k = 2;
        let res = -1;
        assert_eq!(Solution::maximum_energy(energy, k), res);
    }
}
