#[cfg(test)]
mod tests {

    use crate::dd_challenge::*;
    #[test]
    fn test_rotate_digits() {

        assert_eq!(s00788_rotated_digits::count_rotate_digits(10), 4);
    }

    #[test]
    fn test_valid_parentheses() {
        assert_eq!(s00020_valid_parentheses::valid_parentheses("{}()([])"), true);
        assert_eq!(s00020_valid_parentheses::valid_parentheses(""), true);
        assert_eq!(s00020_valid_parentheses::valid_parentheses("{}])"), false);
        assert_eq!(s00020_valid_parentheses::valid_parentheses("{[()}]"), false);
        assert_eq!(s00020_valid_parentheses::valid_parentheses("{([{)}])"), false);
        assert_eq!(s00020_valid_parentheses::valid_parentheses("([)]"), false);
    }

    #[test]
    fn test_generate_parentheses() {

        let result = s00022_generate_parentheses::generate_parentheses(3);

        assert_eq!(result, ["((()))", "(()())", "(())()", "()(())", "()()()"]);

    }

    #[test]
    fn test_check_if_word_is_valid_after_substitution() {
        let tv1 = "aabcbc".to_string();
        let qs = "abc";
        assert_eq!(s01003_check_if_word_is_valid_after_substitution::isValid(tv1, qs), true);

        let tv2 = "abcabcababcc".to_string();
        assert_eq!(s01003_check_if_word_is_valid_after_substitution::isValid(tv2, qs), true);

        let tv3 = "abccba".to_string();
        assert_eq!(s01003_check_if_word_is_valid_after_substitution::isValid(tv3, qs), false);

        let tv4 = "cababc".to_string();
        assert_eq!(s01003_check_if_word_is_valid_after_substitution::isValid(tv4, qs), false);

    }

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses(")(".to_string()), 0);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses("(((((()()".to_string()), 4);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses("((((((((()))".to_string()), 6);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses(")()(((())))(".to_string()), 10);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses("(()(((()".to_string()), 2);
        assert_eq!(s00032_longest_valid_parentheses::longest_valid_parentheses("".to_string()), 0);
    }

    #[test]
    fn test_roman_to_int() {
        assert_eq!(s00013_roman_to_int::roman_to_int("III".to_string()), 3);
        assert_eq!(s00013_roman_to_int::roman_to_int("IV".to_string()), 4);
        assert_eq!(s00013_roman_to_int::roman_to_int("IX".to_string()), 9);
        assert_eq!(s00013_roman_to_int::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(s00013_roman_to_int::roman_to_int("DCXXI".to_string()), 621);
        assert_eq!(s00013_roman_to_int::roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn test_zigzag_conversion() {
        assert_eq!(
            s00006_zigzag_conversion::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI"
        );
        assert_eq!(
            s00006_zigzag_conversion::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR"
        );
        assert_eq!(s00006_zigzag_conversion::convert("A".to_string(), 1), "A");
        assert_eq!(s00006_zigzag_conversion::convert("AY".to_string(), 2), "AY");
    }

    #[test]
    fn test_reverse_words() {
        let mut src = vec!['t','h','e',' ','s','k','y',' ','i','s',' ','b','l','u','e'];
        let tar = vec!['b','l','u','e',' ','i','s',' ','s','k','y',' ','t','h','e'];
        s00186_reverse_words_in_a_string::reverse_words(&mut src);
        assert_eq!(src, tar);
    }

    #[test]
    fn test_reverse_words_541() {
        let origin_str = "abcdefg".to_string();
        let expected_str = "bacdfeg".to_string();
        assert_eq!(s00541_reverse_string::reverse_str(origin_str, 2), expected_str);
        assert_eq!(s00006_zigzag_conversion::convert("AY".to_string(), 2), "AY");
    }
}