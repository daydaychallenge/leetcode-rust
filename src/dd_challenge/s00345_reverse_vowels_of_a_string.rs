use crate::dd_challenge::Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        if s.len() == 0 {
            return s;
        }

        fn is_vowel(c: char) -> bool {
            c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
        }

        let mut left = 0;
        let mut right = s.len() - 1;

        let mut chars: Vec<char> = s.chars().collect();

        while left < right  {
            if !is_vowel(chars[left].to_ascii_lowercase()) {
                left += 1;
            } else {
                if !is_vowel(chars[right].to_ascii_lowercase()) {
                    right -= 1;
                } else {
                    chars.swap(left, right);
                    left += 1;
                    right -= 1;
                }
            }
        }

        chars.iter().collect()

    }


}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_reverse_vowels() {
        assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle".to_string());
        assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede".to_string());
    }
}