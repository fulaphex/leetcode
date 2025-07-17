use std::{cmp, collections};

struct Solution {}
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; k as usize]; k as usize];
        for i in 0..k {
            dp[((nums[0] + i) % k) as usize][(nums[0] % k) as usize] = 1;
        }

        for n in nums.iter().skip(1) {
            let mut upd = collections::HashMap::<(i32, i32), i32>::new();
            let nmod = n % k;
            for a in 0..k {
                let prev = (k + a - nmod) % k;
                let val = cmp::max(
                    dp[a as usize][prev as usize] + 1,
                    dp[a as usize][nmod as usize],
                );
                upd.insert((a, nmod), val);
            }
            for ((a, b), v) in upd {
                dp[a as usize][b as usize] = v;
            }
        }
        *dp.iter().map(|x| x.iter().max().unwrap()).max().unwrap()
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 2;
        let res = 5;
        assert_eq!(Solution::maximum_length(nums, k), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 4, 2, 3, 1, 4];
        let k = 4;
        let res = 4;
        assert_eq!(Solution::maximum_length(nums, k), res);
    }

    #[test]
    fn test3() {
        let nums = vec![
            149646, 361319, 685833, 81324, 554513, 924412, 403755, 705617, 935995, 101980, 460436,
            158197, 640720, 204533, 84422, 592655, 775808, 50842, 28347, 727965, 626681, 469793,
            841307, 563451, 776855, 424174, 953258, 103665, 308806, 748820, 304859, 596480, 680587,
            88418, 880863, 172353, 685914, 35865, 201711, 535719, 862203, 778282, 656494, 632909,
            616005, 918069, 937535, 422968, 349631, 427126, 537829, 190891, 896222, 631662, 351240,
            208770, 44512, 255881, 305112, 199610, 657655, 750607, 326160, 854785, 109026, 919484,
            862944, 774922, 729533, 903656, 715583, 622631, 543134, 742886, 497094, 693349, 806539,
            94940, 873914, 481297, 400209, 200879, 132173, 105389, 37874, 724268, 812821, 944157,
            635707, 72687, 180919, 610368, 910310, 356333, 111179, 363440, 507930, 883929, 416706,
            835565,
        ];
        let k = 567;
        let res = 4;
        assert_eq!(Solution::maximum_length(nums, k), res);
    }
}
