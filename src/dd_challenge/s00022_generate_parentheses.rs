
pub fn do_sth() {
    let r = generate_parentheses(3);

    println!("generate_parentheses(3) is: {:?}", r);
}

/// test case
/// # Example
///


pub fn generate_parentheses(n: i32) -> Vec<String> {
    let mut answer = Vec::new();
    generate(0, 0, String::new(), n, &mut answer);

    answer
}

fn generate(left: i32, right: i32, str: String, n: i32, result: &mut Vec<String>) {
    if left == n && right == n {
         result.push(str.to_string());
    }

    if left < n {
        generate(left + 1, right, str.to_owned() + "(", n, result);
    }

    if right < left {
        generate(left, right + 1, str + ")", n, result);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_parentheses() {

        let result = super::generate_parentheses(3);

        assert_eq!(result, ["((()))", "(()())", "(())()", "()(())", "()()()"]);

    }
}