use crate::dd_challenge::Solution;
use std::str::FromStr;
use std::collections::{HashSet};

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let es = emails.into_iter()
            .map(|word| {
                word.parse::<Email>().unwrap()
            })
            .collect::<HashSet<_>>();
        println!("{:?}", es);
        es.len() as i32
    }

}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Email {
    local: String,
    domain: String,
}

impl Email {
    fn new(local: String, domain: String) -> Self {
        Email {local, domain }
    }
}

impl FromStr for Email {

    type  Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split("@").collect();
        let mut l = s[0];
        let d = s[1];
        l = l.split('+').collect::<Vec<_>>()[0];
        let l = l.replace(".", "");

        Ok(Email::new(l, d.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_num_unique_emails() {
        let input = vec!["test.email+alex@leetcode.com".to_string(),
                         "test.e.mail+bob.cathy@leetcode.com".to_string(),
                         "testemail+david@lee.tcode.com".to_string()];
        let expect = 2;

        assert_eq!(Solution::num_unique_emails(input), expect);
    }
}