
pub fn do_sth() {
    let to_valid = "abccba".to_string();
    let query_str = "abc";
    let r = is_valid(to_valid, query_str);
    println!("{:?}", r);
}

/// step 1: 如果to_valid包含query_str, 从to_valid中去掉query_str后得到新的to_valid
/// step 2: 重复step 1, 直到to_valid末尾,此时to_valid为空即为true
pub fn is_valid(to_valid: String, query_str: &str) -> bool {
    if to_valid.is_empty() {
        return true;
    } else {
        if to_valid.contains(query_str) {
            let to_val = to_valid.replacen(query_str, "", 1);
            println!("{:?}", to_val);
            is_valid(to_val, query_str)
        } else {
            return false;
        }

    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_check_if_word_is_valid_after_substitution_01003() {
        let tv1 = "aabcbc".to_string();
        let qs = "abc";
        assert_eq!(super::is_valid(tv1, qs), true);

        let tv2 = "abcabcababcc".to_string();
        assert_eq!(super::is_valid(tv2, qs), true);

        let tv3 = "abccba".to_string();
        assert_eq!(super::is_valid(tv3, qs), false);

        let tv4 = "cababc".to_string();
        assert_eq!(super::is_valid(tv4, qs), false);
    }
}