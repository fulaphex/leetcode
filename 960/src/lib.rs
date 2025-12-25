impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let strlen = strs[0].len();
        let str_count = strs.len();

        let mut dp = vec![1; strlen + 1];
        // dp[i] - number of columns I can keep and everything is still sorted

        for i in 1..strlen {
            for j in 0..i {
                if (0..str_count).all(|str_idx| {
                    strs[str_idx].as_bytes().get(j).unwrap()
                        <= strs[str_idx].as_bytes().get(i).unwrap()
                }) {
                    dp[i] = dp[i].max(dp[j] + 1);
                    if dp[i] == i {
                        break;
                    }
                }
            }
        }

        (strlen - *dp.iter().max().unwrap()) as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let strs = ["babca", "bbazb"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();
        let res = 3;

        assert_eq!(Solution::min_deletion_size(strs), res);
    }
}
