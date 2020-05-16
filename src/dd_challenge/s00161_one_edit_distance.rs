use crate::dd_challenge::Solution;
use std::cmp::min;

///
/// 对比两个字符串对应位置上的字符，如果遇到不同的时候，这时看两个字符串的长度关系，
/// 如果相等，则比较当前位置后的字串是否相同，如果s的长度大，那么比较s的下一个位置开始的子串，
/// 和t的当前位置开始的子串是否相同，反之如果t的长度大，则比较t的下一个位置开始的子串，
/// 和s的当前位置开始的子串是否相同。如果循环结束，都没有找到不同的字符，
/// 那么此时看两个字符串的长度是否相差1
///
impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let s_len = s_bytes.len();
        let t_len = t_bytes.len();

        let loop_len = min(s_len, t_len);

        for idx in 0..loop_len {
            if s_bytes[idx] != t_bytes[idx] {
                return if s_len == t_len {
                    s_bytes[(idx + 1)..] == t_bytes[(idx + 1)..]
                } else if s_len > t_len {
                    s_bytes[(idx + 1)..] == t_bytes[idx..]
                } else {
                    s_bytes[idx..] == t_bytes[(idx + 1)..]
                }
            }
        }

        return (s_len as i32 - t_len as i32).abs() == 1;

    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_is_one_edit_distance() {
        assert!(Solution::is_one_edit_distance("ab".to_string(), "abc".to_string()));
        assert!(!Solution::is_one_edit_distance("ab".to_string(), "abcd".to_string()));
        assert!(!Solution::is_one_edit_distance("cab".to_string(), "ad".to_string()));
        assert!(Solution::is_one_edit_distance("1203".to_string(), "1213".to_string()));
    }
}