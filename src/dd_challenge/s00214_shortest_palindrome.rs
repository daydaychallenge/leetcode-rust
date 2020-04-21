
pub fn do_sth() {
    let s = "ababab".to_string();
    let res = get_prefix_table(s.as_bytes());
    let res1 = shortest_palindrome(s);
    println!("{:?}", res);
    println!("{:?}", res1);
}
pub fn shortest_palindrome(s: String) -> String {

    let prefix_table = get_prefix_table(s.as_bytes());

    let mut left = 0;

    let s_vec = s.as_bytes();

    for i in (0..s_vec.len()).rev() {
        while left > 0 && s_vec[left] != s_vec[i] {
            left = prefix_table[left - 1];
        }

        if s_vec[left] == s_vec[i] {
            left += 1;

        }
    }

    s[left..].chars().rev().collect::<String>() + &s

}

fn get_prefix_table(s:&[u8]) -> Vec<usize> {
    // Use KMP
    // the values in table is 1-base index, in order to uss 'size-t'
    // so '1' correspond to 's[0]'.
    // Orm 'l' correspond to 's[l - 1]', vice versa 's[l]' correspond to 'l + 1'
    let len = s.len();
    let mut table = vec![0; len];
    let mut left = 0;

    for i in 1..s.len() {
        while left > 0 && s[left] != s[i] {
            left = table[left - 1];
        }

        if s[left] == s[i] {
            left += 1;
        }

        table[i] = left;
    }

    table
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shortest_palindrome() {
        let s1 = "aacecaaa".to_string();
        let exp1 = "aaacecaaa";

        assert_eq!(super::shortest_palindrome(s1), exp1);

        let s2 = "abcd".to_string();
        let exp2 = "dcbabcd";
        assert_eq!(super::shortest_palindrome(s2), exp2);
    }
}