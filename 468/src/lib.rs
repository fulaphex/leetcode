use regex::Regex;

struct Solution {}

impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        // result constants
        let v4 = String::from("IPv4");
        let v6 = String::from("IPv6");
        let neither = String::from("Neither");

        // ipv4 regex
        let v4_group = r"(\d{1,3})";
        let re4 = Regex::new(&format!("^{}$", vec![v4_group; 4].join(r"\.").as_str())).unwrap();

        let res4 = re4.captures(&query_ip);
        if res4.is_some() {
            return if res4
                .unwrap()
                .iter()
                .skip(1)
                .map(|x| x.unwrap().as_str())
                .all(|x| {
                    (!x.starts_with("0") || x == "0") && (x.parse::<i32>().unwrap_or(256) < 256)
                }) {
                v4
            } else {
                neither
            };
        }

        // ipv6 regex
        let v6_octet = r"([\da-fA-F]{1,4})";
        let re6 = Regex::new(&format!("^{}$", vec![v6_octet; 8].join(r":").as_str())).unwrap();

        let res6 = re6.captures(&query_ip);
        if res6.is_some() {
            return v6;
        }

        // didn't match ipv4 or ipv6 - returning neither
        return neither;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let query_ip = String::from("172.16.254.1");
        let res = String::from("IPv4");
        assert_eq!(Solution::valid_ip_address(query_ip), res);
    }

    #[test]
    fn test2() {
        let query_ip = String::from("2001:0db8:85a3:0:0:8A2E:0370:7334");
        let res = String::from("IPv6");
        assert_eq!(Solution::valid_ip_address(query_ip), res);
    }

    #[test]
    fn test3() {
        let query_ip = String::from("256.256.256.256");
        let res = String::from("Neither");
        assert_eq!(Solution::valid_ip_address(query_ip), res);
    }

    #[test]
    fn test4() {
        let query_ip = String::from("01.01.01.01");
        let res = String::from("Neither");
        assert_eq!(Solution::valid_ip_address(query_ip), res);
    }

    #[test]
    fn test5() {
        let query_ip = String::from("0.0.0.0");
        let res = String::from("IPv4");
        assert_eq!(Solution::valid_ip_address(query_ip), res);
    }

    #[test]
    fn test6() {
        let query_ip = String::from("172.16.254:1");
        let res = String::from("Neither");
        assert_eq!(Solution::valid_ip_address(query_ip), res);
    }
}
