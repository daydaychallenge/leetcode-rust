// use super::Solution;
use crate::dd_challenge::Solution;

use std::collections::*;

/// https://leetcode.com/problems/word-ladder-ii/discuss/40477/Super-fast-Java-solution-(two-end-BFS)
impl Solution {
    pub fn find_ladder(begin_word: String, end_word: String, word_list: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: Vec<Vec<String>> = vec![];
        let mut dict: HashSet<String> = HashSet::new();

        for s in word_list {
            dict.insert(s);
        }

        if !dict.contains(&end_word) {
            return ans;
        }

        let mut set1 = HashSet::<String>::new();
        let mut set2 = HashSet::<String>::new();

        set1.insert(begin_word.clone());
        set2.insert(end_word.clone());

        let mut map = HashMap::<String, Vec<String>>::new();
        bfs(&mut map, &set1, &set2, &mut dict, false);

        let mut path = Vec::<String>::new();
        path.push(begin_word.clone());
        dfs(&mut ans, &mut path, &mut map, begin_word.clone(), end_word.clone());

        return ans;

        fn bfs(map: &mut HashMap<String, Vec<String>>, set1: &HashSet<String>, set2: &HashSet<String>,
               dict: &mut HashSet<String>, flip: bool) {
            if set1.is_empty() {
                return;
            }

            if set1.len() > set2.len() {
                bfs(map, set2, set1, dict, !flip);
                return ;
            }

            let mut done = false;
            for s in set1 { dict.remove(s); }
            for s in set2 { dict.remove(s); }
            let mut next = HashSet::<String>::new();
            for str in set1 {
                let mut chs = str.chars().collect::<Vec<char>>();
                let n = chs.len();
                for i in 0..n {
                    let t = chs[i];

                    for ch in 'a' as u8..='z' as u8 {
                        if chs[i] != ch as char {
                            chs[i] = ch as char;
                            let word = chs.iter().collect::<String>();
                            let key = if flip { word.clone() } else { str.clone() };
                            let val = if flip { str.clone() } else { word.clone() };
                            let mut list = if let Some(x) = map.get(&key.clone()) {
                                x.clone()
                            } else {
                                Vec::<String>::new()
                            };
                            if set2.contains(&word) {
                                done = true;
                                list.push(val.clone());
                                map.insert(key.clone(), list.clone());
                            }
                            if !done && dict.contains(&word) {
                                next.insert(word);
                                list.push(val.clone());
                                map.insert(key.clone(), list.clone());
                            }
                        }
                    }
                    chs[i] = t;
                }
            }
            if !done { bfs(map, set2, &next, dict, !flip) }
        }

        fn dfs(ans: &mut Vec<Vec<String>>, path: &mut Vec<String>, map: &mut HashMap<String, Vec<String>>, start: String, end: String) {
            if start == end {
                ans.push(path.clone());
                return;
            }
            if !map.contains_key(&start) { return; }

            let x = map.get(&start).cloned().unwrap();
            for next in x {
                path.push(next.clone());
                dfs(ans, path, map, next.clone(), end.clone());
                path.pop();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_find_ladder() {
        assert_eq!(
            Solution::find_ladder("hit".to_string(), "cog".to_string(),
                                  vec!["hot".to_string(),"dot".to_string(),"dog".to_string(),
                                      "lot".to_string(),"log".to_string(),"cog".to_string()]),
                                      vec![
                                          vec!["hit".to_string(),"hot".to_string(),"dot".to_string(),"dog".to_string(),"cog".to_string()],
                                          vec!["hit".to_string(),"hot".to_string(),"lot".to_string(),"log".to_string(),"cog".to_string()]
                                        ]);

        let vec_empty: Vec<Vec<String>> = vec![];
        assert_eq!(
            Solution::find_ladder("hit".to_string(), "cog".to_string(),
                                  vec!["hot".to_string(),"dot".to_string(),"dog".to_string(),
                                      "lot".to_string(),"log".to_string()]), vec_empty);
    }
}