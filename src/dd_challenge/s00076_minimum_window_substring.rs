use crate::dd_challenge::Solution;
use std::collections::{HashSet, HashMap, VecDeque};

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if t.is_empty() {
            return "".to_string();
        }

        let target = t.as_bytes().into_iter().fold(HashMap::new(), |mut a, e| {
            a.entry(e).and_modify(|v| *v += 1).or_insert(1);
            a
        });

        let mut qualify = t.as_bytes().into_iter().map(|&e| e).collect::<HashSet<u8>>();
        let mut rec = HashMap::new();

        let s = s.as_bytes();

        let mut q = VecDeque::new();

        let mut ret: &[u8] = &[];

        for (i, c) in s.into_iter().enumerate() {
            if !target.contains_key(c) {
                continue;
            }

            q.push_back(i);

            if *rec.entry(c).and_modify(|e| *e += 1).or_insert(1) >= *target.get(c).unwrap() {
                qualify.remove(c);
            }

            while !q.is_empty() {
                let bef_len = q.len();
                let qc = &s[*q.front().unwrap()];
                rec.entry(qc).and_modify(|e| {
                    if *e > *target.get(qc).unwrap() {
                        q.pop_front();
                        *e -= 1;
                    }
                });

                if bef_len == q.len() {
                    break;
                }
            }

            if qualify.is_empty() {
                if ret.len() == 0 || q.back().unwrap() - q.front().unwrap() + 1 < ret.len() {
                    ret = &s[*q.front().unwrap()..=*q.back().unwrap()];
                }
            }
        }

        String::from_utf8(ret.to_owned()).unwrap()
    }

}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_min_window() {
        assert_eq!(Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC".to_string());
    }
}