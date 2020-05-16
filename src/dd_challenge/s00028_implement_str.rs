use crate::dd_challenge::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            - 1
        } else if needle.is_empty() {
            0
        } else {
            let h_s = haystack.as_bytes();
            let n_s = needle.as_bytes();

            let h_len = h_s.len();
            let n_len = n_s.len();
            let mut h_idx = 0usize;
            let mut n_idx = 0usize;

            while h_idx < h_len {

                if h_s[h_idx] != n_s[n_idx] {
                    h_idx += 1;
                    n_idx = 0;
                } else {
                    h_idx += 1;
                    n_idx += 1;

                    if n_idx == n_len {
                        return (h_idx - n_len) as i32;
                    }
                }
            }

            return -1;
            /*match haystack.find(&needle) {
                Some(i) => i as i32,
                None => -1,
            }*/
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_str_str() {
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(Solution::str_str("aaaaa".to_string(), "bba".to_string()), -1);
    }
}