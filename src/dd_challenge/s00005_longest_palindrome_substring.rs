
pub fn do_sth() {
    let s = "Hello World!";
    let start = 0;
    let end = 5;
    let ss = &s[start..end];
    println!("{:?}", ss);

    let s1 = "abcdcbfk";
    let r1 = longest_palindrome(s1.to_string());
    println!("{:?}", r1);



}
///
/// 给定一个字符串 s，找到 s 中最长的回文子串。你可以假设 s 的最大长度为 1000。
///
/// 示例 1：
///
/// 输入: "babad"
/// 输出: "bab"
/// 注意: "aba" 也是一个有效答案。
/// 示例 2：
///
/// 输入: "cbbd"
/// 输出: "bb"
///
///

pub fn longest_palindrome(s: String) -> String {

    let ss: Vec<char> = s.chars().collect();
    let len = ss.len();

    if len <= 1{
        return s;
    }

    let (mut begin, mut end, mut max) =  (0, 0, 0);

    let mut left;
    let mut right;

    for i in 0..len {
        left = i;
        right = i;

        while left > 0 && ss[left - 1] == ss[i] {
            left -= 1;
        }

        while right < len - 1 && ss[right + 1] == ss[i] {
            right += 1;
        }

        while left > 0 && right < len - 1 && ss[left - 1] == ss[right + 1] {
            left -= 1;
            right += 1;
        }

        if right - left > max {
            begin = left;
            end = right;
            max = end - begin;
        }

    }

    s[begin..end + 1].to_string()

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_longest_palindrome() {
        let s1 = "babad".to_string();
        let exp1 = "bab".to_string();
        assert_eq!(super::longest_palindrome(s1), exp1);

        let s2 = "cbbd".to_string();
        let exp2 = "bb".to_string();
        assert_eq!(super::longest_palindrome(s2), exp2);
    }
}