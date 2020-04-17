
pub fn do_sth() {
    let s = " Hello  world!    ".to_string();
    let sp = s.trim().split(' ');
    for ss in sp {
       println!("{:?}", ss);
    }
}

pub fn reverse_words(s: String) -> String {
    s.split_whitespace()
        .rev()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reverse_words() {
        let s1 = "the sky is blue".to_string();
        let ep1 = "blue is sky the".to_string();
        assert_eq!(super::reverse_words(s1), ep1);

        let s2 = "  hello world!  ".to_string();
        let ep2 = "world! hello".to_string();
        assert_eq!(super::reverse_words(s2), ep2);

        let s3 = "a good   example ".to_string();
        let ep3 = "example good a".to_string();
        assert_eq!(super::reverse_words(s3), ep3);
    }
}