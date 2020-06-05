use crate::dd_challenge::Solution;

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut cnt = [0; 26];

        for ch in s.bytes() {
            cnt[(ch - b'a') as usize] += 1;
        }

        for (i, ch) in s.bytes().enumerate() {
            if cnt[(ch - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_first_uniq_char() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}

pub fn do_sth() {
    Solution::first_uniq_char("leetcode".to_string());
    Solution::first_uniq_char("loveleetcode".to_string());
}