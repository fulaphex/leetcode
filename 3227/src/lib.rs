impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        let mut counts = vec![0; 30];
        for c in s.bytes() {
            counts[(c - b'a') as usize] += 1;
        }
        return counts[(b'a' - b'a') as usize]
            + counts[(b'e' - b'a') as usize]
            + counts[(b'i' - b'a') as usize]
            + counts[(b'o' - b'a') as usize]
            + counts[(b'u' - b'a') as usize]
            > 0;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("leetcoder");
        let res = true;
        assert_eq!(Solution::does_alice_win(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from("bbcd");
        let res = false;
        assert_eq!(Solution::does_alice_win(s), res);
    }
}
