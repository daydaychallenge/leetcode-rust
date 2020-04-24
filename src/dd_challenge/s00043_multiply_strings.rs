

 /// [43] Multiply Strings
 ///
 /// Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.
 ///
 /// Example 1:
 ///
 ///
 /// Input: num1 = "2", num2 = "3"
 /// Output: "6"
 ///
 /// Example 2:
 ///
 ///
 /// Input: num1 = "123", num2 = "456"
 /// Output: "56088"
 ///
 ///
 /// Note:
 ///
 /// <ol>
 /// 	The length of both num1 and num2 is < 110.
 /// 	Both num1 and num2 contain only digits 0-9.
 /// 	Both num1 and num2 do not contain any leading zero, except the number 0 itself.
 /// 	You must not use any built-in BigInteger library or convert the inputs to integer directly.
 /// </ol>
 ///

pub fn multiply(num1: String, num2: String) -> String {
    let mut res = vec![0; num1.len() + num2.len()];

    for (i, c1) in num1.chars().rev().enumerate() {
        for (j, c2) in num2.chars().rev().enumerate() {
            let l = c1.to_digit(10).unwrap();
            let r = c2.to_digit(10).unwrap();

            let lo = (l * r + res[i + j]) % 10;
            let hi = (l * r + res[i + j]) / 10;
            res[i + j] = lo;
            res[i + j + 1] += hi;

            println!("{:?} and {:?} !", i, j);
            println!("{:?}", res);
        }
    }

    while res.len() > 1 && res.last() == Some(&0) {
        res.pop();
    }

    res.into_iter().rev().map(|c| c.to_string()).collect::<String>()
}

pub fn do_sth() {
    let res = multiply("123".to_string(), "456".to_string());
    println!("{:?}", res);

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_multiply() {
        let res1 = super::multiply("2".to_string(), "3".to_string());
        assert_eq!(res1, "6");

        let res2 = super::multiply("123".to_string(), "456".to_string());
        assert_eq!(res2, "56088");
    }
}