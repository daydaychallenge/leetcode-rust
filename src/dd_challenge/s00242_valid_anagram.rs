use crate::dd_challenge::Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut ss = s.as_bytes().to_vec();
        ss.sort();
        let mut tt = t.as_bytes().to_vec();
        tt.sort();
        ss == tt
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_is_anagram() {
        assert!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}