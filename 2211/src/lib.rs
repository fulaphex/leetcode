impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let x = directions.trim_start_matches('L').trim_end_matches('R');
        x.matches('L').count() as i32 + x.matches('R').count() as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let directions = String::from("RLRSLL");
        let res = 5;
        assert_eq!(Solution::count_collisions(directions), res);
    }
}
