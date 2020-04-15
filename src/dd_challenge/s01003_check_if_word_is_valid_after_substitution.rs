
pub fn do_sth() {
    let to_valid = "abccba".to_string();
    let query_str = "abc";
    let r = isValid(to_valid, query_str);
    println!("{:?}", r);
}

/// step 1: 如果to_valid包含query_str, 从to_valid中去掉query_str后得到新的to_valid
/// step 2: 重复step 1, 直到to_valid末尾,此时to_valid为空即为true
pub fn isValid(to_valid: String, query_str: &str) -> bool {
    if to_valid.is_empty() {
        return true;
    } else {
        if to_valid.contains(query_str) {
            let to_val = to_valid.replacen(query_str, "", 1);
            println!("{:?}", to_val);
            isValid(to_val, query_str)
        } else {
            return false;
        }

    }

}