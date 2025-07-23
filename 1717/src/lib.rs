struct Solution {}

impl Solution {
    pub fn _max(chars: &[char], x: i32, y: i32) -> i32 {
        let mut res = 0;

        let (mut acount, mut bcount, mut a2count) = (0, 0, 0);
        for &c in chars {
            if c == 'a' {
                if acount == 0 && bcount == 0 {
                    acount += 1;
                } else {
                    a2count += 1;
                }
            } else {
                // c = 'b';
                if acount > 0 {
                    res += x;
                    acount -= 1;
                } else if a2count > 0 {
                    res += x;
                    a2count -= 1;
                } else {
                    // acount = 0; a2count = 0
                    bcount += 1;
                }
            }
        }
        if bcount > 0 {
            assert_eq!(acount, 0);
        }
        if acount > 0 {
            assert_eq!(bcount, 0)
        }
        res += y * bcount.min(a2count);

        return res;
    }
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let mut sections: Vec<Vec<char>> = vec![];
        let mut curr_section: Vec<char> = vec![];
        for letter in s.chars().into_iter().chain([' ']) {
            // println!("char: {}", letter);
            if letter == 'a' {
                if x >= y {
                    curr_section.push(letter);
                } else {
                    curr_section.push('b');
                }
            } else if letter == 'b' {
                if x >= y {
                    curr_section.push(letter);
                } else {
                    curr_section.push('a');
                }
            } else {
                if curr_section.len() > 0 {
                    sections.push(curr_section);
                }
                curr_section = vec![];
            }
        }
        let mut res = 0;
        for section in sections {
            res += Self::_max(&section, x.max(y), y.min(x));
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = String::from("cdbcbbaaabab");
        let (x, y) = (4, 5);
        let res = 19;
        assert_eq!(Solution::maximum_gain(s, x, y), res);
    }

    #[test]
    fn test2() {
        let s = String::from("aabbaaxybbaabb");
        let (x, y) = (5, 4);
        let res = 20;
        assert_eq!(Solution::maximum_gain(s, x, y), res);
    }
}
