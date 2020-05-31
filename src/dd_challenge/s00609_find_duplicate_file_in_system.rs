use crate::dd_challenge::Solution;
use std::collections::HashMap;

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {

        let mut key_path = Vec::new();

        for p in paths {
            let dir_files: Vec<&str> = p.split(' ').collect();
            let dir = dir_files[0].clone();

            for idx in 1..dir_files.len() {
                let path_key = &dir_files[idx];
                let i_idx = path_key.find('(').unwrap();
                let j_idx = path_key.find(')').unwrap();
                let path: String = path_key.chars().take(i_idx).collect();
                let key: String = path_key.chars().skip(i_idx).take(j_idx - i_idx).collect();

                key_path.push((key, dir.to_string() + "/" + path.as_str()))
            }

        }

        let init: HashMap<String, Vec<String>> = HashMap::new();
        let res = key_path.into_iter().fold(init, | mut acc, cur| {
            let (k, v) = cur;
            acc.entry(k)
                .and_modify(|vv|  vv.push(v.clone()) )
                .or_insert(vec![v]);
            acc
        });

        let mut r: Vec<Vec<String>> = vec![];

        for vv in res.values().into_iter() {
            r.push(vv.clone());
        };

        r

    }
}

pub fn do_sth() {
    let h: HashMap<&str, Vec<String>> = HashMap::new();

    let inputs = vec![("one", 1), ("two", 2), ("three", 3), ("one", 11), ("one", 111), ("three", 33)];
    let res = inputs.into_iter().fold(h, |mut acc, cur| {
        acc.entry(cur.0)
            .and_modify(|v| v.push(cur.1.to_string()))
            .or_insert(vec![(cur.1).to_string()]);
        acc
    });

    println!("{:?}", res);

    let origin = vec!["root/a 1.txt(abcd) 2.txt(efgh)".to_string(), "root/c 3.txt(abcd)".to_string(),
                      "root/c/d 4.txt(efgh)".to_string(), "root 4.txt(efgh)".to_string()];
    let res2 = Solution::find_duplicate(origin);

    println!("{:?}", res2);
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_find_duplicate() {
        let origin = vec!["root/a 1.txt(abcd) 2.txt(efgh)".to_string(), "root/c 3.txt(abcd)".to_string(),
                          "root/c/d 4.txt(efgh)".to_string(), "root 4.txt(efgh)".to_string()];
        let mut expected = vec![vec!["root/a/1.txt".to_string(), "root/c/3.txt".to_string()],
                            vec!["root/a/2.txt".to_string(), "root/c/d/4.txt".to_string(), "root/4.txt".to_string()],
                            ];
        assert_eq!(Solution::find_duplicate(origin).sort_unstable(), expected.sort_unstable());
    }
}