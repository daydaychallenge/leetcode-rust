use crate::dd_challenge::Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {

        moves.bytes().fold((0, 0), |acc, cur| {
            match cur {
                b'U' => (acc.0, acc.1 + 1),
                b'D' => (acc.0, acc.1 - 1),
                b'L' => (acc.0 + 1, acc.1),
                b'R' => (acc.0 - 1, acc.1),
                _ => panic!("Invalid input!"),
            }
        }) == (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_judge_circle() {
        let input1 = "UD".to_string();
        assert!(Solution::judge_circle(input1));
        assert!(!Solution::judge_circle("LL".to_string()));
        assert!(Solution::judge_circle("LLUUDDRRUDRL".to_string()));
        assert!(!Solution::judge_circle("LLUUDDRRUDRU".to_string()));
    }
}