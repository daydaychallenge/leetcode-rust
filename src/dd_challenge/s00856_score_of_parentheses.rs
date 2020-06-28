use super::Solution;

impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut res = 0;
        let mut balance = 0;
        let mut last = false;

        for x in s.chars().into_iter() {
            if x == '(' {
                balance += 1;
                last = true;
            } else {
                balance -= 1;
                if last {
                    res += 1 << balance;
                }
                last = false;
            }
        }

        res
    }
}

#[test]
fn test_score_of_parentheses() {
    assert_eq!(Solution::score_of_parentheses("()".to_string()), 1);
    assert_eq!(Solution::score_of_parentheses("(())".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses("()()".to_string()), 2);
    assert_eq!(Solution::score_of_parentheses("(()(()))".to_string()), 6);
}