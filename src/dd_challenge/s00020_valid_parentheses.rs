
use std::collections::{VecDeque, HashMap};

const LEFT_PARENTHESES: &[char] = &['[', '{', '('];
const RIGHT_PARENTHESE: &[char] = &[']', '}', ')'];

pub fn do_sth() {
    let p1 = "{}()";
    let r1 = valid_parentheses(p1);
    println!("the {:?} is valid parentheses: {:?}", p1, r1);

    let p2 = "{(})";
    let r2 = valid_parentheses(p2);
    println!("the {:?} is valid parentheses: {:?}", p2, r2);
}

/// step 1: 迭代字符串，遇到左括号入栈
/// step 2: 遇到右括号，左括号栈出栈并比较左右括号是否配对，如果是，继续迭代，否则返回false
/// step 3: 迭代完成后，如果左括号栈为空，返回true，否则为false
pub fn valid_parentheses(str: &str) -> bool {
    let mut parentheses: HashMap<char, char> = HashMap::new();
    parentheses.insert(')', '(');
    parentheses.insert(']', '[');
    parentheses.insert('}', '{');

    let mut left_parenthese = VecDeque::new();
    for c in str.chars() {
        if LEFT_PARENTHESES.contains(&c) {
           left_parenthese.push_back(c)
        } else {
            if parentheses.get(&c) == left_parenthese.pop_back().as_ref() {
                println!("Find the parentheses: right is {:?}", c);
            } else {
                return false;
            }
        }
    }

    left_parenthese.is_empty()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_valid_parentheses() {
        assert_eq!(super::valid_parentheses("{}()([])"), true);
        assert_eq!(super::valid_parentheses(""), true);
        assert_eq!(super::valid_parentheses("{}])"), false);
        assert_eq!(super::valid_parentheses("{[()}]"), false);
        assert_eq!(super::valid_parentheses("{([{)}])"), false);
        assert_eq!(super::valid_parentheses("([)]"), false);
    }

}