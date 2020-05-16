use crate::dd_challenge::Solution;
use std::collections::*;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

        strs.into_iter().fold(BTreeMap::new(), |mut map, s| {
            let mut key = BTreeMap::new();
            s.as_bytes().iter().for_each(|&c| *key.entry(c).or_insert(0) += 1);
            map.entry(key).or_insert(vec![]).push(s);
            map
        }).into_iter().map(|(_, v)| v).collect()

        /*let mut res = HashMap::new();

        for s in strs {
            let mut k: Vec<u8> = s.bytes().collect();
            k.sort_unstable();
            res.entry(k).or_insert(vec![]).push(s)
        }

        res.into_iter().map(|(_, v)| v).collect()*/
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_group_anagrams() {
        let input: Vec<String> = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(),
                                      "ate".to_string(), "nat".to_string(), "bat".to_string()];
        assert_eq!(Solution::group_anagrams(input), vec![
            vec!["bat".to_string()],
            vec!["eat".to_string(),"tea".to_string(),"ate".to_string()],
            vec!["tan".to_string(),"nat".to_string()]
        ])
    }
}