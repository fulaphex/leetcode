impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let (mut prev, mut res) = (0, 0);
        for row in bank {
            let curr = row.chars().filter(|x| x == &'1').count();
            res += curr * prev;
            if curr > 0 {
                prev = curr;
            }
        }
        res as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bank: Vec<String> = ["011001", "000000", "010100", "001000"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let res = 8;
        assert_eq!(Solution::number_of_beams(bank), res);
    }

    #[test]
    fn test2() {
        let bank: Vec<String> = ["000", "111", "000"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let res = 0;
        assert_eq!(Solution::number_of_beams(bank), res);
    }
}
