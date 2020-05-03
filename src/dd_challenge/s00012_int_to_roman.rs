use crate::dd_challenge::Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {

        let mut stack = vec![(1, "I"),
                             (4, "IV"),
                             (5, "V"),
                             (9, "IX"),
                             (10, "X"),
                             (40, "XL"),
                             (50, "L"),
                             (90, "XC"),
                             (100, "C"),
                             (400, "CD"),
                             (500, "D"),
                             (900, "CM"),
                             (1000, "M")
        ];

        let mut res: Vec<&str> = vec![];
        let mut input = num;
        while let Some(&(x, s)) = stack.last() {
            if x <= input {
                input -= x;
                res.push(s);
            } else {
                stack.pop();
            }
        }

        res.join("")
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_int_to_roman() {
        assert_eq!(Solution::int_to_roman(1), "I".to_string());
        assert_eq!(Solution::int_to_roman(14), "XIV".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
        assert_eq!(Solution::int_to_roman(3999), "MMMCMXCIX".to_string());
    }
}