use crate::dd_challenge::Solution;

impl Solution {
    pub fn defang_ip_addr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_defang_ip_addr() {
        assert_eq!(Solution::defang_ip_addr("1.1.1.1".to_string()), "1[.]1[.]1[.]1");
        assert_eq!(Solution::defang_ip_addr("255.100.50.0".to_string()), "255[.]100[.]50[.]0");

    }
}