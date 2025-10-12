impl Solution {
    pub fn calculate(s: String) -> i32 {
        let (mut numbers, mut operators) = (vec![], vec![]);
        let mut num = 0;
        for c in s.chars() {
            if c == ' ' {
                continue;
            } else if c >= '0' && c <= '9' {
                num = 10 * num + (c as u8 - '0' as u8) as i32;
            } else {
                operators.push(c);
                numbers.push(num);
                num = 0;
            }
        }
        numbers.push(num);

        let (mut new_numbers, mut new_operators) = (vec![numbers[0]], vec![]);
        for (op, y) in std::iter::zip(operators, numbers.into_iter().skip(1)) {
            let last_idx = new_numbers.len() - 1;
            if op == '+' || op == '-' {
                new_numbers.push(y);
                new_operators.push(op);
            } else if op == '*' {
                new_numbers[last_idx] *= y;
            } else if op == '/' {
                new_numbers[last_idx] /= y;
            } else {
                panic!("unknown operator");
            }
        }
        (numbers, operators) = (new_numbers, new_operators);

        let first_num = numbers[0];
        return std::iter::zip(operators, numbers.into_iter().skip(1)).fold(
            first_num,
            |acc, (op, y)| if op == '+' { acc + y } else { acc - y },
        );
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = String::from("3+2*2");
        let res = 7;
        assert_eq!(Solution::calculate(s), res);
    }

    #[test]
    fn test2() {
        let s = String::from(" 3/2 ");
        let res = 1;
        assert_eq!(Solution::calculate(s), res);
    }

    #[test]
    fn test3() {
        let s = String::from(" 3+5 / 2 ");
        let res = 5;
        assert_eq!(Solution::calculate(s), res);
    }
}
