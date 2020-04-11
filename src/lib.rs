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
}