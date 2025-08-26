struct Solution {}

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        return dimensions
            .iter()
            .map(|x| (x[0] * x[0] + x[1] * x[1], x[0] * x[1]))
            .max()
            .unwrap()
            .1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let dimensions = [[9, 3], [8, 6]].iter().map(|x| Vec::from(x)).collect();
        let res = 48;
        assert_eq!(Solution::area_of_max_diagonal(dimensions), res);
    }

    #[test]
    fn test2() {
        let dimensions = [[3, 4], [4, 3]].iter().map(|x| Vec::from(x)).collect();
        let res = 12;
        assert_eq!(Solution::area_of_max_diagonal(dimensions), res);
    }
}
