use crate::dd_challenge::Solution;

impl Solution {
    pub fn is_matching_wildcard(s: String, p: String) -> bool {
        let new_s = "#".to_string() + &s;
        let new_p = "#".to_string() + &p;

        let s_chars : Vec<char> = new_s.chars().collect();
        let p_chars : Vec<char> = new_p.chars().collect();

        let s_len = s_chars.len();
        let p_len = p_chars.len();

        let mut s_idx = 1;
        let mut p_idx = 1;

        let mut star_idx = 0;
        let mut s_tmp_idx = 0;

        while s_idx < s_len {
            if p_idx < p_len && (p_chars[p_idx] == '?' || p_chars[p_idx] == s_chars[s_idx]) {
                s_idx += 1;
                p_idx += 1;
            }
            else if p_idx < p_len && p_chars[p_idx] == '*' {
                star_idx = p_idx;
                s_tmp_idx = s_idx;
                p_idx += 1;
            }
            else if star_idx == 0 {
                return false;
            }
            else {
                p_idx = star_idx + 1;
                s_idx = s_tmp_idx + 1;
                s_tmp_idx = s_idx;
            }
        }

        for i in p_idx..p_len {
            if p_chars[i] != '*' {
                return false;
            }
        }

        return true;
    }


}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_is_match() {
        assert!(!Solution::is_matching_wildcard("aa".to_string(), "a".to_string()));
        assert!(Solution::is_matching_wildcard("aa".to_string(), "*".to_string()));
        assert!(!Solution::is_matching_wildcard("cb".to_string(), "?a".to_string()));
        assert!(Solution::is_matching_wildcard("adceb".to_string(), "*a*b".to_string()));
        assert!(!Solution::is_matching_wildcard("acdcb".to_string(), "a*c?b".to_string()));
        assert!(Solution::is_matching_wildcard("".to_string(), "".to_string()));
        assert!(Solution::is_matching_wildcard("abcdekks".to_string(), "?*".to_string()));
    }
}

