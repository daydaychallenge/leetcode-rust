use crate::dd_challenge::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        format!("{:b}", i128::from_str_radix(a.as_str(), 2).unwrap() + i128::from_str_radix(b.as_str(), 2).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_add_binary() {
        let a = "1010";
        let b = "1011";
        let res = Solution::add_binary(a.to_string(), b.to_string());
        let exp = "10101";

        assert_eq!(res, exp);
    }
}