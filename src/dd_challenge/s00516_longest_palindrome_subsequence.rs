use std::cmp::max;

pub fn longest_palindrome(s: String) -> i32 {

    let n = s.len();
    let mut dp = vec![vec![0; n]; n];

    for len in 1..n + 1 {
        for i in 0..n-len + 1 {
            let j = i + len - 1;
            if i == j {
                dp[i][j] = 1;
                continue;
            }

            dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);

            if s.as_bytes()[i] == s.as_bytes()[j] {
                dp[i][j] = dp[i + 1][j - 1]+ 2;
            }

            println!("{:?}", dp[0][j]);
        }
    }

    return dp[0][n - 1];
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_longest_palindrome() {
        let s1 = "bbbab".to_string();
        let exp1 = 4;
        assert_eq!(super::longest_palindrome(s1), exp1);

        let s2 = "cbba".to_string();
        let exp2 = 2;
        assert_eq!(super::longest_palindrome(s2), exp2);
    }

}