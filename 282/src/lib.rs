/*
 transitions:
 - starting 0 + 1 * first
 - x + (y * z)
   - concating z ->
      - if y == 0 -> don't concat
      - if y > 0 -> x + (10*y+z)
      - else -> x + (10*y-z)
   - adding c -> (x+y*z, 1, c)
   - subtracting c -> becomes (x+y*z, -1, c)
   - multiplying by z -> becomes (x, y, 10*z+c)
*/

impl Solution {
    const ADD: char = '+';
    const SUB: char = '-';
    const MULT: char = '*';

    #[inline]
    fn parse_char(c: &char) -> i64 {
        return c.to_digit(10).unwrap() as i64;
    }

    fn parse(
        num: &[char],
        target: i64,
        x: i64,
        y: i64,
        z: i64,
        acc: &mut Vec<char>,
        res: &mut Vec<String>,
    ) {
        if num.len() == 0 {
            let total = x + y * z;

            if total == target {
                res.push(String::from_iter(acc.iter()));
            }
            return;
        }

        let acc_len = acc.len();
        let first_num = Self::parse_char(&num[0]);

        acc.push(' '); // operator that will be set later
        acc.push(num[0]);

        // add
        {
            let (nx, ny, nz) = (x + y * z, 1, first_num);
            acc[acc_len] = Self::ADD;
            Self::parse(&num[1..], target, nx, ny, nz, acc, res);
        }

        // sub
        {
            let (nx, ny, nz) = (x + y * z, -1, first_num);
            acc[acc_len] = Self::SUB;
            Self::parse(&num[1..], target, nx, ny, nz, acc, res);
        }

        // mult
        {
            let (nx, ny, nz) = (x, y * z, first_num);
            acc[acc_len] = Self::MULT;
            Self::parse(&num[1..], target, nx, ny, nz, acc, res);
        }

        // only concatenation left - not setting the operator and just setting the digit
        acc.pop();
        acc[acc_len] = num[0];

        // concat
        {
            let (nx, ny, nz) = (x, y, 10 * z + first_num);
            if z != 0 {
                Self::parse(&num[1..], target, nx, ny, nz, acc, res);
            }
        }

        acc.pop();

        return;
    }

    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let chars: Vec<char> = num.chars().collect();
        let first_digit = Self::parse_char(&chars[0]);
        let mut res = vec![];
        Self::parse(
            &chars[1..],
            target as i64,
            0,
            1,
            first_digit,
            &mut vec![chars[0]],
            &mut res,
        );
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let num = String::from("123");
        let target = 6;
        let res = ["1+2+3", "1*2*3"]
            .iter()
            .map(|&x| String::from(x))
            .collect::<Vec<String>>();
        assert_eq!(Solution::add_operators(num, target), res);
    }

    #[test]
    fn test2() {
        let num = String::from("232");
        let target = 8;
        let res = ["2+3*2", "2*3+2"]
            .iter()
            .map(|&x| String::from(x))
            .collect::<Vec<String>>();
        assert_eq!(Solution::add_operators(num, target), res);
    }

    #[test]
    fn test3() {
        let num = String::from("3456237490");
        let target = 9191;
        let res: Vec<String> = vec![];
        assert_eq!(Solution::add_operators(num, target), res);
    }

    #[test]
    fn test4() {
        let num = String::from("2147483648");
        let target = -2147483648;
        let res: Vec<String> = vec![];
        assert_eq!(Solution::add_operators(num, target), res);
    }

    #[test]
    fn test5() {
        let num = String::from("105");
        let target = 5;
        let res = ["1*0+5", "10-5"]
            .iter()
            .map(|&x| String::from(x))
            .collect::<Vec<String>>();
        assert_eq!(Solution::add_operators(num, target), res);
    }
}
