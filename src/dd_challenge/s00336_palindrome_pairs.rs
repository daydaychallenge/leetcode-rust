use std::collections::HashMap;

pub fn do_sth() {
    let os1 = vec!["bat".to_string(), "tab".to_string(), "cat".to_string()];
    let exp1 = vec![(0, 1), (1, 0)];
    let res1 = palindrome_pairs(os1);
    println!("{:?}", res1);

    let os2 = vec!["abcd".to_string(), "dcba".to_string(), "lls".to_string(),
                   "s".to_string(), "sssll".to_string()];
    let exp2 = vec![(0, 1), (1, 0), (2, 4), (3, 2)];
    assert_eq!(palindrome_pairs(os2), exp2);

    let os3 = "abc".to_string();
    let os31 = "cba".to_string();
    let res3 = is_palindrome(&os3, &os31);
    println!("{:?}", res3);

    let os4 = "abc".to_string();
    let os41 = "ba".to_string();
    let res4 = is_palindrome(&os4, &os41);
    println!("{:?}", res4);

}

pub fn palindrome_pairs(words: Vec<String>) -> Vec<(i32, i32)> {

    let len = words.len();
    let mut res: Vec<(i32, i32)> = Vec::new();

    let mut words1 = words.clone();
    let mut words2 = words.clone();
    for i in 0..len {
        for j in 0..len {
            if i != j && is_palindrome(&words1[i], &words2[j]) {
                res.push((i as i32, j as i32));
            }
        }
    }

    res
}

fn is_palindrome(val1: &String, val2: &String) -> bool {
    let ss = val1.to_owned() + val2;
    let len = ss.len();
    for i in 0..(len / 2) {

        if ss.as_bytes()[i] != ss.as_bytes()[len - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_palindrome_pairs() {
        let os1 = vec!["bat".to_string(), "tab".to_string(), "cat".to_string()];
        let exp1 = vec![(0, 1), (1, 0)];
        let res1 = super::palindrome_pairs(os1);
        assert_eq!(res1, exp1);

        let os2 = vec!["abcd".to_string(), "dcba".to_string(), "lls".to_string(),
                       "s".to_string(), "sssll".to_string()];
        let exp2 = vec![(0, 1), (1, 0), (2, 4), (3, 2)];
        assert_eq!(super::palindrome_pairs(os2), exp2);
    }
}
