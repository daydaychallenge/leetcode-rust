use crate::dd_challenge::Solution;

///
/// step 1: 字符串从左到右比较是否重复,不重复连在一起
/// step 2: 一旦字符重复的话,记下当前不重复的内容的长度,并保存,重新开始计算子字符串,直到完成整个字符串
/// step 3: 返回其中子字符串长度最长的长度值
///
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let len = s.len();
        if len <= 1 {
            return len as i32;
        }

        let chars: Vec<char> = s.chars().collect();
        let mut res: Vec<usize> = Vec::new();

        let mut sub_s: Vec<char> = vec![];

        for c in chars {
            if sub_s.contains(&c) {
                res.push(sub_s.len());
                sub_s.clear();
                sub_s.push(c);
            } else {
                sub_s.push(c)
            }

        }
        res.push(sub_s.len());

        *res.iter().max().unwrap() as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_length_of_longest_substring() {
        //assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring("bbbb".to_string()), 1);
        assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    }
}