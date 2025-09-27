struct Solution {}

impl Solution {
    fn eval(nums: &[f64], operations: &[char]) -> bool {
        // println!("evaling {:?} {:?}", nums, operations);
        if operations.len() == 0 {
            // println!("result: {}", nums[0]);
            return (nums[0] - 24.).abs() < 1e-5;
            // return nums[0] == 24.;
        }
        for (idx, &op) in operations.iter().enumerate() {
            let el = match op {
                '+' => nums[idx] + nums[idx + 1],
                '-' => nums[idx] - nums[idx + 1],
                '*' => nums[idx] * nums[idx + 1],
                '/' => nums[idx] / nums[idx + 1],
                _ => panic!(),
            };
            if Self::eval(
                &[&nums[..idx], &[el], &nums[idx + 2..]].concat(),
                &[&operations[..idx], &operations[idx + 1..]].concat(),
            ) {
                return true;
            }
        }
        return false;
    }
    fn generate_operations(acc: &mut Vec<char>, left: i32, nums: &Vec<f64>) -> bool {
        if left == 0 {
            return Self::eval(&nums, &acc);
        }
        for op in "+-*/".chars() {
            acc.push(op);
            if Self::generate_operations(acc, left - 1, nums) {
                return true;
            }
            acc.pop();
        }

        return false;
    }
    fn permutations(cards: &[i32], acc: &mut Vec<f64>) -> bool {
        if cards.len() == 0 {
            // println!("acc: {:?}", acc);
            return Self::generate_operations(&mut vec![], 3, acc);
        }
        for (idx, c) in cards.iter().enumerate() {
            acc.push(*c as f64);
            if Self::permutations(&[&cards[..idx], &cards[idx + 1..]].concat(), acc) {
                return true;
            }
            acc.pop();
        }
        return false;
    }
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        return Self::permutations(&cards, &mut vec![]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let cards = vec![4, 1, 8, 7];
        assert!(Solution::judge_point24(cards));
    }

    #[test]
    fn test2() {
        let cards = vec![1, 2, 1, 2];
        assert!(!Solution::judge_point24(cards));
    }
}
