impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut res = vec![];
        let mut curr = 0;

        for (idx, c) in expression.chars().enumerate() {
            if c == '+' || c == '-' || c == '*' {
                let op = match c {
                    '+' => |x, y| x + y,
                    '-' => |x, y| x - y,
                    '*' => |x, y| x * y,
                    _ => unreachable!(),
                };

                let left_options = Self::diff_ways_to_compute(expression[..idx].to_string());
                let right_options = Self::diff_ways_to_compute(expression[idx + 1..].to_string());
                for &x in &left_options {
                    for &y in &right_options {
                        res.push(op(x, y));
                    }
                }
            } else {
                curr = 10 * curr + c.to_digit(10).unwrap();
            }
        }

        if res.is_empty() {
            res.push(curr as i32);
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let expression = String::from("2-1-1");
        let res = vec![0, 2];
        let mut out = Solution::diff_ways_to_compute(expression);
        out.sort_unstable();

        assert_eq!(out, res);
    }

    #[test]
    fn test2() {
        let expression = String::from("2*3-4*5");
        let res = vec![-34, -14, -10, -10, 10];
        let mut out = Solution::diff_ways_to_compute(expression);
        out.sort_unstable();

        assert_eq!(out, res);
    }
}
