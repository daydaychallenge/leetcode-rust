use crate::dd_challenge::Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        is_match(s.as_bytes(), p.as_bytes())
    }

}

fn is_match(s: &[u8], p: &[u8]) -> bool {
    match parse(p) {
        (Pattern::Empty, _) => s.is_empty(),
        (Pattern::Single(c), subp) => is_match_single(s, c, subp),
        (Pattern::Repeatable(c), subp) => is_match_single(s, c, p) || is_match(s, subp),
    }
}

fn is_match_single(s: &[u8], to_match: u8, p: &[u8]) -> bool {
    match s.split_first() {
        Some((c, s)) if to_match == b'.' || to_match == *c => is_match(s, p),
        _ => false,
    }
}

/// Parser part:
enum Pattern {
    Empty,
    Single(u8),
    Repeatable(u8),
}

/// Returns the parsed pattern and the next pattern to parse
fn parse(p: &[u8]) -> (Pattern, &[u8]) {
    match p.split_first() {
        None => (Pattern::Empty, p),
        Some((h, t)) => match t.split_first() {
            Some((b'*', tt)) => (Pattern::Repeatable(*h), tt),
            _ => (Pattern::Single(*h), t),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_is_match() {
        assert!(!Solution::is_match("aa".to_string(), "a".to_string()));
        assert!(Solution::is_match("aa".to_string(), "a*".to_string()));
        assert!(Solution::is_match("ab".to_string(), ".*".to_string()));
        assert!(Solution::is_match("aab".to_string(), "c*a*b".to_string()));
        assert!(!Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()));
        assert!(Solution::is_match("".to_string(), "".to_string()));
        assert!(Solution::is_match("abcdekks".to_string(), ".*".to_string()));
    }
}