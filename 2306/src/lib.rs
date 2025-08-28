impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut map: std::collections::HashMap<char, std::collections::HashSet<String>> =
            std::collections::HashMap::new();

        for idea in ideas {
            let pref = idea.chars().next().unwrap();
            let mut suff = idea;
            suff.remove(0);

            map.entry(pref)
                .or_insert(std::collections::HashSet::new())
                .insert(suff);
        }

        let mut res = 0;

        for (idx, v1) in map.values().enumerate() {
            for v2 in map.values().skip(idx + 1) {
                let union_size = v1.intersection(v2).count();
                res += 2 * ((v1.len() - union_size) as i64) * ((v2.len() - union_size) as i64);
            }
        }

        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ideas = ["coffee", "donuts", "time", "toffee"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let res = 6;
        assert_eq!(Solution::distinct_names(ideas), res);
    }
}
