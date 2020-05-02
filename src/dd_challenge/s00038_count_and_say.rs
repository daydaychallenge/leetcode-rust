use crate::dd_challenge::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n <= 1 {
            return n.to_string();
        }

        let prv_res = Self::count_and_say(n - 1).chars().collect::<Vec<char>>();
        let mut res = vec![];

        let mut pre_char = &prv_res[0];
        let mut count = 1;

        for c in &prv_res[1..] {
            if c == pre_char {
                count += 1;
            } else {
                res.push(format!("{}{}", count, pre_char));
                count = 1;
            }

            pre_char = c;
        }

        res.push(format!("{}{}", count, pre_char));
        res.join("")

    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_count_and_say() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
        assert_eq!(Solution::count_and_say(5), "111221".to_string());
        assert_eq!(Solution::count_and_say(6), "312211".to_string());
        assert_eq!(Solution::count_and_say(7), "13112221".to_string());
    }

}