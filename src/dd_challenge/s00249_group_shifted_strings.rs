use crate::dd_challenge::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut res_map: HashMap<String, Vec<String>> = HashMap::new();
        for s in strings {
            let mut key = "".to_string();
            let ss = s.as_bytes();
            for i in 1..ss.len() {
                key += (((26 + ss[i] - ss[i - 1]) % 26).to_string() + ",").as_str();

            }

            res_map.entry(key)
                .or_insert(Vec::new())
                .push(s)
        }

        let mut res: Vec<Vec<String>> = vec![];

        for (_, v) in res_map {
            res.push(v);
        }

        res
    }
}

pub fn do_sth() {
    let o = vec!["abc".to_string(), "bcd".to_string(), "acef".to_string(),
                 "xyz".to_string(), "az".to_string(), "ba".to_string(), "a".to_string(),
                 "z".to_string()];
    Solution::group_strings(o);
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_group_strings() {
        let o = vec!["abc".to_string(), "bcd".to_string(), "acef".to_string(),
                     "xyz".to_string(), "az".to_string(), "ba".to_string(), "a".to_string(),
                     "z".to_string()];

        let mut exp = vec![
            vec!["abc".to_string(),"bcd".to_string(),"xyz".to_string()],
            vec!["az".to_string(),"ba".to_string()],
            vec!["acef".to_string()],
            vec!["a".to_string(),"z".to_string()]
        ];

        assert_eq!(Solution::group_strings(o).sort_unstable(), exp.sort_unstable());
    }
}