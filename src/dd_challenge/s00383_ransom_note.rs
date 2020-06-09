use crate::dd_challenge::Solution;

impl Solution {
    pub fn can_construct(ransom: String, magazine: String) -> bool {
        let needed = find_letter_occurences(&ransom);
        let available = find_letter_occurences(&magazine);

        needed.iter().zip(available.iter()).all(|(a, b)| a <= b)
    }
}

fn find_letter_occurences(s: &str) -> [i32; 26] {
    s.bytes().fold([0; 26], |mut acc, cur| {
        let current = cur - b'a';
        acc[current as usize] += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_can_construct() {
        assert!(!Solution::can_construct("a".to_string(), "b".to_string()));
        assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
        assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
    }
}