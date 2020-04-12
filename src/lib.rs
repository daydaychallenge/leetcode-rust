pub mod dd_challenge;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    use crate::dd_challenge::*;
    #[test]
    fn test_rotate_digits() {

        assert_eq!(rotated_digits::count_rotate_digits(10), 4);
    }

    #[test]
    fn test_valid_parentheses() {
        assert_eq!(valid_parentheses::valid_parentheses("{}()([])"), true);
        assert_eq!(valid_parentheses::valid_parentheses(""), true);
        assert_eq!(valid_parentheses::valid_parentheses("{}])"), false);
        assert_eq!(valid_parentheses::valid_parentheses("{[()}]"), false);
        assert_eq!(valid_parentheses::valid_parentheses("{([{)}])"), false);
        assert_eq!(valid_parentheses::valid_parentheses("([)]"), false);
    }

    #[test]
    fn test_generate_parentheses() {

        let result = generate_parentheses::generate_parentheses(3);

        assert_eq!(result, ["((()))", "(()())", "(())()", "()(())", "()()()"]);

    }

    #[test]
    fn test_check_if_word_is_valid_after_substitution() {
        let tv1 = "aabcbc".to_string();
        let qs = "abc";
        assert_eq!(check_if_word_is_valid_after_substitution::isValid(tv1, qs), true);

        let tv2 = "abcabcababcc".to_string();
        assert_eq!(check_if_word_is_valid_after_substitution::isValid(tv2, qs), true);

        let tv3 = "abccba".to_string();
        assert_eq!(check_if_word_is_valid_after_substitution::isValid(tv3, qs), false);

        let tv4 = "cababc".to_string();
        assert_eq!(check_if_word_is_valid_after_substitution::isValid(tv4, qs), false);

    }
}