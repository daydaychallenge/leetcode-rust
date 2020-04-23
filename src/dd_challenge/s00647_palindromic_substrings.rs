
pub fn do_sth() {

    let s = "abba".to_string();
    let res = count_substrings(s);

    println!("The count substrings is : {:?}", res);
}

/// Manacher Algorithm
/// step 1: add '$''#' into string
/// The character '$' here is just to prevent overbounds
/// there is an even palindrome 'abba' and odd palindrome 'opxpo' in 's="abahopxpo"',
/// which are converted into '#a#b#b#a#' and '#o#p#x#p#o#',
/// and the length is converted into odd
///
pub fn count_substrings(s: String) -> i32 {

    let mut new_str = vec!['$', '#'];
    for ch in s.chars() {
        new_str.push(ch);
        new_str.push('#');
    }
    new_str.push('\0');

    let len = new_str.len();

    // Define a secondary array p[],
    // where p[i] represents the radius of the longest palindrome centered on i.
    let mut p = vec![0usize; len];

    // Define two variables, 'mx' and 'idx'
    // 'mx' represents the right boundary of the longest palindrome centered on 'idx',
    // which is 'mx = id + p[i]'
    let mut idx = 0usize;
    let mut mx = 0usize;

    for i in 1..len - 1 {
        if i < mx {
            p[i] = p[2 * idx - i].min(mx - i);
        } else {
            p[i] = 1;
        }

        while new_str[i - p[i]] == new_str[i + p[i]] {
            p[i] += 1;
        }

        if mx < i + p[i] {
            idx = i;
            mx = i + p[i];
        }
    }

    println!("The mx array is: {:?}", p);

    let sum: i32 = p.iter().map(|x| *x as i32 / 2).sum();

    sum

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_count_substrings() {
        let s1 = "abc".to_string();
        let exp1 = 3;
        assert_eq!(super::count_substrings(s1), exp1);

        let s2 = "aaa".to_string();
        let exp2 = 6;
        assert_eq!(super::count_substrings(s2), exp2);
    }
}