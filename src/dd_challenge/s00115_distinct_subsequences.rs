use crate::dd_challenge::Solution;

/// https://leetcode.com/problems/distinct-subsequences/discuss/236467/Rust-Bottom-Up-DP-with-0-ms-and-O(N)-space
///
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s: Vec<char> = s.chars().collect();
        let mut cache = vec![0; s.len()];

        for (i, ch) in t.chars().into_iter().enumerate() {
            let mut acc = 0;
            // first char initialization
            if i == 0 {
                for j in 0..s.len() {
                    if ch == s[j] { cache[j] = 1 }
                }
                continue;
            }

            for j in 0..s.len() {
                let new_acc = acc + cache[j];
                cache[j] = if s[j] == ch { acc } else { 0 };
                acc = new_acc;
            }
        }

        cache.into_iter().fold(0, |acc, x| acc + x)
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_num_distinct() {
        assert_eq!(Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()), 3);
        assert_eq!(Solution::num_distinct("babgbag".to_string(), "bag".to_string()), 5);
    }
}