#[cfg(test)]
mod tests {

    use crate::dd_challenge::*;
    #[test]
    fn test_rotate_digits() {

        assert_eq!(rotated_digits_00788::count_rotate_digits(10), 4);
    }

    #[test]
    fn test_valid_parentheses() {
        assert_eq!(valid_parentheses_00020::valid_parentheses("{}()([])"), true);
        assert_eq!(valid_parentheses_00020::valid_parentheses(""), true);
        assert_eq!(valid_parentheses_00020::valid_parentheses("{}])"), false);
        assert_eq!(valid_parentheses_00020::valid_parentheses("{[()}]"), false);
        assert_eq!(valid_parentheses_00020::valid_parentheses("{([{)}])"), false);
        assert_eq!(valid_parentheses_00020::valid_parentheses("([)]"), false);
    }

    #[test]
    fn test_generate_parentheses() {

        let result = generate_parentheses_00022::generate_parentheses(3);

        assert_eq!(result, ["((()))", "(()())", "(())()", "()(())", "()()()"]);

    }

    #[test]
    fn test_check_if_word_is_valid_after_substitution() {
        let tv1 = "aabcbc".to_string();
        let qs = "abc";
        assert_eq!(check_if_word_is_valid_after_substitution_01003::isValid(tv1, qs), true);

        let tv2 = "abcabcababcc".to_string();
        assert_eq!(check_if_word_is_valid_after_substitution_01003::isValid(tv2, qs), true);

        let tv3 = "abccba".to_string();
        assert_eq!(check_if_word_is_valid_after_substitution_01003::isValid(tv3, qs), false);

        let tv4 = "cababc".to_string();
        assert_eq!(check_if_word_is_valid_after_substitution_01003::isValid(tv4, qs), false);

    }

    #[test]
    fn test_longest_valid_parentheses() {
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses(")(".to_string()), 0);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses("(((((()()".to_string()), 4);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses("((((((((()))".to_string()), 6);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses("()".to_string()), 2);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses(")()(((())))(".to_string()), 10);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses("(()(((()".to_string()), 2);
        assert_eq!(longest_valid_parentheses_00032::longest_valid_parentheses("".to_string()), 0);
    }

    #[test]
    fn test_roman_to_int() {
        assert_eq!(roman_to_int_00013::roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int_00013::roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int_00013::roman_to_int("IX".to_string()), 9);
        assert_eq!(roman_to_int_00013::roman_to_int("MCMXCIV".to_string()), 1994);
        assert_eq!(roman_to_int_00013::roman_to_int("DCXXI".to_string()), 621);
        assert_eq!(roman_to_int_00013::roman_to_int("LVIII".to_string()), 58);
    }
}