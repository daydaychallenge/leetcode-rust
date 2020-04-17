
pub fn do_sth() {

}

pub fn reverse_str(s: String, k: i32) -> String {
    let k: usize = k as usize;
    let mut idx: usize = 0;
    let mut is_reverse = true;
    let mut res: String = "".to_string();
    let s_len = s.len();
    while idx < s_len {
        if is_reverse {
            let tmp = s.chars().skip(idx).take(k).collect::<String>()
                .chars().rev().collect::<String>();
            res += &tmp;
        } else {
            let ks = s.chars().skip(idx).take(k).collect::<String>();
            res += &ks;
        }

        idx += k;

        is_reverse = !is_reverse;
    }

    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reverse_str() {
        let origin_str = "abcdefg".to_string();
        let expected_str = "bacdfeg".to_string();
        assert_eq!(super::reverse_str(origin_str, 2), expected_str);
    }
}