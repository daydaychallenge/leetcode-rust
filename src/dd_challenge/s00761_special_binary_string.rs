use crate::dd_challenge::Solution;

impl Solution {
    pub fn make_largest_special(s: String) -> String {
        let mut res = vec![];
        let (mut i, mut cnt) = (0usize, 0i32);

        for (j, c) in s.chars().enumerate() {
            cnt += if c == '1' { 1 } else { -1 };
            if cnt == 0 {
                res.push("1".to_string() + &Solution::make_largest_special(s[i+1..j].to_string()) + &"0");
                i = j + 1;
            }
        }

        res.sort_unstable_by(|a, b| b.cmp(&a));
        res.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_make_largest_special() {
        assert_eq!(Solution::make_largest_special("11011000".to_string()), "11100100".to_string());
    }
}