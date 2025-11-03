impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut prev = ' ';
        let (mut curr_sum, mut curr_max) = (0, 0);

        for (col, &time) in std::iter::zip(colors.chars(), needed_time.iter()) {
            if col != prev {
                prev = col;
                res += curr_sum - curr_max;
                (curr_sum, curr_max) = (0, 0);
            }

            curr_sum += time;
            curr_max = curr_max.max(time);
        }

        res += curr_sum - curr_max;
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let colors = String::from("abaac");
        let needed_time = vec![1, 2, 3, 4, 5];
        let res = 3;

        assert_eq!(Solution::min_cost(colors, needed_time), res);
    }

    #[test]
    fn test2() {
        let colors = String::from("abc");
        let needed_time = vec![1, 2, 3];
        let res = 0;

        assert_eq!(Solution::min_cost(colors, needed_time), res);
    }

    #[test]
    fn test3() {
        let colors = String::from("aabaa");
        let needed_time = vec![1, 2, 3, 4, 1];
        let res = 2;

        assert_eq!(Solution::min_cost(colors, needed_time), res);
    }
}
