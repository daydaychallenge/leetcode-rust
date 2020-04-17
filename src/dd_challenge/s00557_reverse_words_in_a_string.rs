use std::str::Chars;

pub fn reverse_words(s: String) -> String {
    if s.is_empty() {
        return s;
    }

    let mut s: Vec<char> = s.chars().collect();

    let mut s_len = s.len();
    let mut start = 0;

    while start < s_len {
        let mut end = start;
        while end + 1 < s_len && s[end + 1] != ' ' {
            end += 1;
        }

        let new_idx = if end + 1 >= s_len {
            end + 1
        } else {
            end + 2
        };

        reverse(&mut s, start, end);

        start = new_idx;
    }

    s.into_iter().collect::<String>()

}

fn reverse(s: &mut Vec<char>, left: usize, right: usize) {

    let mut start = left;
    let mut end = right;
    while start < end {
        s.swap(start, end);
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reverse_words() {
        let origin_str = "Let's take LeetCode contest".to_string();
        let expected_str = "s'teL ekat edoCteeL tsetnoc".to_string();
        assert_eq!(super::reverse_words(origin_str), expected_str);
    }
}