impl Solution {
    pub fn distinct_names(ideas: Vec<String>) -> i64 {
        let mut map: std::collections::HashMap<String, u32> = std::collections::HashMap::new();

        for idea in ideas {
            let pref = idea.chars().next().unwrap();
            let pref_shift = (pref as u8) - ('a' as u8);
            let mut suff = idea;
            suff.remove(0);

            let mut val = *map.get(&suff).unwrap_or(&0);
            val |= 1 << pref_shift;
            map.insert(suff, val);
        }

        let mut res = 0;
        let mut vals: Vec<u32> = map.values().cloned().collect();
        vals.sort();

        for i1 in 0..vals.len() {
            let v1 = vals[i1];
            let nv1 = !v1;
            let (mut subval, mut prev) = (0, 0);

            for i2 in i1 + 1..vals.len() {
                let v2 = vals[i2];
                if v2 != prev {
                    subval =
                        2 * ((v1 & !v2).count_ones() as i64) * ((v2 & nv1).count_ones() as i64);
                    prev = v2;
                }
                res += subval;
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
