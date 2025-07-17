struct Solution {}

impl Solution {
    pub fn combinations_no_copy(
        vals: &[char],
        length: usize,
        previous: Option<char>,
        pattern: &[char],
    ) -> Vec<Vec<char>> {
        if length == 0 {
            return vec![vec![]];
        }
        let mut res: Vec<Vec<char>> = vec![];

        for (idx, &el) in vals.iter().enumerate() {
            let subvals = vec![&vals[..idx], &vals[idx + 1..]].concat();
            if previous.is_some() {
                if pattern[0] == 'U' && previous.unwrap() > el {
                    continue;
                }
                if pattern[0] == 'D' && previous.unwrap() < el {
                    continue;
                }
            }

            let subpattern = match previous {
                Some(_) => &pattern[1..],
                None => &pattern[..],
            };
            for sub_comb in Self::combinations_no_copy(&subvals, length - 1, Some(el), subpattern) {
                let comb: Vec<char> = [el].iter().map(|&x| x).chain(sub_comb).collect::<Vec<_>>();
                res.push(comb);
            }
        }
        return res;
    }

    pub fn smallest_number(pattern: String) -> String {
        let chars_slice = &("123456789".chars().collect::<Vec<_>>());
        // println!("char slice = {:?}", chars_slice);
        let combinations = Self::combinations_no_copy(
            &chars_slice,
            pattern.len() + 1,
            None,
            &pattern.chars().collect::<Vec<_>>(),
        );
        // println!("combinations = {:?}", combinations);
        return combinations[0].iter().collect::<String>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let pattern = String::from("IIIDIDDD");
        assert_eq!(
            Solution::smallest_number(pattern),
            String::from("123549876")
        );
    }

    #[test]
    fn test2() {
        let pattern = String::from("DDD");
        assert_eq!(Solution::smallest_number(pattern), String::from("4321"));
    }

    #[test]
    fn test3() {
        let pattern = String::from("IDIDIDDI");
        assert_eq!(
            Solution::smallest_number(pattern),
            String::from("132548769")
        );
    }
}
