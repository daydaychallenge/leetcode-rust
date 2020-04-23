use std::collections::HashMap;

pub fn can_permute_palindrome(s: String) -> bool {
    let mut dict = HashMap::<char, usize>::new();

    for c in s.chars() {
        dict.entry(c)
            .and_modify(|x| *x = 0)
            .or_insert(1);
    }

    dict.values().filter(|x| *x > &1usize).count() < 2
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_permute_palindrome() {
        assert!(super::can_permute_palindrome("code".to_string()));
        assert!(super::can_permute_palindrome("aab".to_string()));
        assert!(super::can_permute_palindrome("carerac".to_string()));
    }
}