pub mod dd_challenge;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_rotate_digits() {
        use crate::dd_challenge::*;
        assert_eq!(rotated_digits::count_rotate_digits(10), 4);
    }
}