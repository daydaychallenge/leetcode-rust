use crate::dd_challenge::Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        match s.split_whitespace().last(){
            Some(ss) => ss.len() as i32,
            None => 0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_length_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }
}