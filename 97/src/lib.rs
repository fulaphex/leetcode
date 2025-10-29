impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            return false;
        }
        let (c1, c2, c3) = (s1.as_bytes(), s2.as_bytes(), s3.as_bytes());
        let mut dp = vec![vec![0; s2.len() + 1]; 2];

        for i in 0..=s1.len() {
            for j in 0..=s2.len() {
                if j > 0 {
                    dp[i % 2][j] = dp[i % 2][j]
                        .max(dp[i % 2][j - 1] + if c3[i + j - 1] == c2[j - 1] { 1 } else { 0 });
                }
                if i > 0 {
                    dp[i % 2][j] = dp[i % 2][j]
                        .max(dp[(i + 1) % 2][j] + if c3[i + j - 1] == c1[i - 1] { 1 } else { 0 });
                }
            }
        }

        dp[s1.len() % 2][s2.len()] == s3.len() as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (s1, s2, s3) = (
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbcbcac"),
        );
        let res = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), res);
    }

    #[test]
    fn test2() {
        let (s1, s2, s3) = (
            String::from("aabcc"),
            String::from("dbbca"),
            String::from("aadbbbaccc"),
        );
        let res = false;
        assert_eq!(Solution::is_interleave(s1, s2, s3), res);
    }

    #[test]
    fn test3() {
        let (s1, s2, s3) = (String::from(""), String::from(""), String::from(""));
        let res = true;
        assert_eq!(Solution::is_interleave(s1, s2, s3), res);
    }
}
