impl Solution {
    fn justify(line: Vec<String>, max_width: usize, total_length: usize) -> String {
        let gap_count = line.len() - 1;
        let space_count = max_width - total_length;
        let gaps: Vec<_> = (0..gap_count)
            .map(|idx| {
                " ".repeat(
                    space_count / gap_count
                        + if idx < (space_count % gap_count) {
                            1
                        } else {
                            0
                        },
                )
            })
            .collect();
        assert_eq!(gaps.len() + 1, line.len());
        let mut x = vec![];
        let (mut line_iter, mut gap_iter) =
            (line.into_iter().peekable(), gaps.into_iter().peekable());
        loop {
            if line_iter.peek().is_some() {
                x.push(line_iter.next().unwrap());
            }
            if gap_iter.peek().is_some() {
                x.push(gap_iter.next().unwrap());
            }
            if line_iter.peek().is_none() && gap_iter.peek().is_none() {
                break;
            }
        }
        return x.join("");
    }

    fn left_align(mut line: Vec<String>, max_width: usize, total_length: usize) -> String {
        let gap_count = line.len() - 1;
        let right_pad = max_width - total_length - gap_count;
        if right_pad > 0 {
            line.push(" ".repeat(right_pad - 1));
        }
        return line.join(" ");
    }

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let umax_width = max_width as usize;
        let (mut row, mut total_length, mut size) =
            (vec![words[0].clone()], words[0].len(), words[0].len());
        let mut rows = vec![];

        for word in words.into_iter().skip(1) {
            if size + (word.len()) + 1 <= umax_width {
                total_length += word.len();
                size += (word.len()) + 1;
                row.push(word);
            } else {
                if row.len() == 1 {
                    rows.push(Self::left_align(row, umax_width, total_length));
                } else {
                    rows.push(Self::justify(row, umax_width, total_length));
                }
                total_length = word.len();
                size = word.len();
                row = vec![word];
            }
        }
        rows.push(Self::left_align(row, umax_width, total_length));

        return rows;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let words: Vec<String> = [
            "This",
            "is",
            "an",
            "example",
            "of",
            "text",
            "justification.",
        ]
        .into_iter()
        .map(|x| String::from(x))
        .collect();
        let max_width = 16;
        let result: Vec<String> = ["This    is    an", "example  of text", "justification.  "]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        assert_eq!(Solution::full_justify(words, max_width), result);
    }

    #[test]
    fn test2() {
        let words: Vec<String> = ["What", "must", "be", "acknowledgment", "shall", "be"]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let max_width = 16;
        let result: Vec<String> = ["What   must   be", "acknowledgment  ", "shall be        "]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        assert_eq!(Solution::full_justify(words, max_width), result);
    }

    #[test]
    fn test3() {
        let words: Vec<String> = [
            "Science",
            "is",
            "what",
            "we",
            "understand",
            "well",
            "enough",
            "to",
            "explain",
            "to",
            "a",
            "computer.",
            "Art",
            "is",
            "everything",
            "else",
            "we",
            "do",
        ]
        .into_iter()
        .map(|x| String::from(x))
        .collect();
        let max_width = 20;
        let result: Vec<String> = [
            "Science  is  what we",
            "understand      well",
            "enough to explain to",
            "a  computer.  Art is",
            "everything  else  we",
            "do                  ",
        ]
        .into_iter()
        .map(|x| String::from(x))
        .collect();
        assert_eq!(Solution::full_justify(words, max_width), result);
    }

    #[test]
    fn test4() {
        let words: Vec<String> = ["Listen", "to", "many,", "speak", "to", "a", "few."]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        let max_width = 6;
        let result: Vec<String> = ["Listen", "to    ", "many, ", "speak ", "to   a", "few.  "]
            .into_iter()
            .map(|x| String::from(x))
            .collect();
        assert_eq!(Solution::full_justify(words, max_width), result);
    }
}
