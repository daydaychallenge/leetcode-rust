use crate::dd_challenge::Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let ss: Vec<char> = s.chars()
            .filter(|c| c.is_alphanumeric() || c.is_alphabetic()).collect();

        let len = ss.len();

        let (mut left, mut right) = (0, len - 1);

        while left  < right {
            if !ss[left].eq_ignore_ascii_case(&ss[right]) {
                return false;
            }
            left += 1;
            right -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()));
        assert!(!Solution::is_palindrome("race a car".to_string()))
    }
}