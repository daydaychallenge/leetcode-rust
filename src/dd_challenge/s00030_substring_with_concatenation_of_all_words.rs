use crate::dd_challenge::Solution;
use std::collections::HashMap;

/// https://stevenbai.top/rust-leetcode/%E6%AF%8F%E5%A4%A9%E4%B8%80%E9%81%93rust-leetcode2019-11-22/
///
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut m = HashMap::new();
        if words.len() == 0 {
            return Vec::new(); //空的字符串,不用找
        }
        if words[0].len() == 0 {
            return Vec::new(); //字符串的长度是0,也不用找了
        }
        let sub_len = words[0].len();
        let mut total_len = 0;
        for s in words.iter() {
            *m.entry(s.as_bytes()).or_insert(0) += 1;
            total_len += sub_len;
        }
        let mut i = 0;
        let mut res = Vec::new();
        let s = s.as_bytes();
        while i + total_len <= s.len() {
            let mut m2 = m.clone();
            let mut j = i;
            while m2.len() > 0 && j + sub_len <= s.len() {
                println!("i={},j={}", i, j);
                let r = j..(j + sub_len);
                let sub = &s[r];
                if m2.contains_key(sub) {
                    let e = m2.get_mut(sub).expect("must have");
                    *e -= 1;
                    if *e == 0 {
                        m2.remove(sub);
                    }
                } else {
                    break;
                }
                j += sub_len;
            }
            //如果全部都匹配到了,这时候m2肯定空了.
            //如果匹配到了,可以考虑跳的更多,
            //比如words=[aa,bb,cc]
            //如果i匹配到了,比如s[i:i+sub_len]=cc,那么只需要看s[i+sub_len*2:i+sub_len*3]是否是cc
            //如果是cc,自然可以继续匹配,如果不是,还要分两种情况
            //1. s[i+sub_len*2:i+sub_len*3]是否在m中,如果不在,跳过
            //2.如果在,谨慎起见,只能继续从i+sub_len开始匹配,
            if m2.is_empty() {
                res.push(i as i32);
            }
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert_eq!(
            Solution::find_substring(
                String::from("barfoothefoobarman"),
                vec![String::from("foo"), String::from("bar")]
            ),
            vec![0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("word")
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("wordgoodgoodgoodbestword"),
                vec![
                    String::from("word"),
                    String::from("good"),
                    String::from("best"),
                    String::from("good")
                ]
            ),
            vec![8]
        );
        assert_eq!(
            Solution::find_substring(
                String::from("ababaab"),
                vec![String::from("ab"), String::from("ba"), String::from("ba")]
            ),
            vec![1]
        );
    }
}