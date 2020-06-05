use crate::dd_challenge::Solution;

/**
解题思路
分情况来处理遍历，num 表示当前的数字，curRes 表示当前的结果，res 为最终的结果，op 为操作符号，初始化为 '+'。
当遇到数字的时候，将 num 自乘以 10 并加上这个数字，这是由于可能遇到多位数，所以每次要乘以 10。

如果遇到运算符号，或者是最后一个位置的字符时，我们根据 上一个op 的值对 num 进行分别的加减乘除的处理，结果保存到 当前结果curRes 中。
然后再次判读如果 上一个op 是加或减，或者是最后一个位置的字符时，将 当前结果curRes 加到最终结果 res 中，并且 curRes 重置为0。
最后将当前运算字符c赋值给 op（注意这里只有当时最后一个位置的字符时，才有可能不是运算符号，不过也无所谓，因为遍历已经结束），num 也要重置为0

*/

impl Solution {
    pub fn calculate(s: String) -> i32 {

        let (mut res, mut cur_res, mut num, n) = (0i32, 0i32, 0i32, s.len() as i32);

        let mut op = '+';

        for (idx, c) in s.chars().enumerate() {
            if c.is_alphanumeric() && c >= '0' && c <= '9' {
                num = num * 10 + c.to_digit(10).unwrap() as i32;
            }

            if c == '+' || c == '-' || c == '*' || c == '/' || idx == (n - 1) as usize {
                match op {
                    '+' => cur_res += num,
                    '-' => cur_res -= num,
                    '*' => cur_res *= num,
                    '/' => cur_res /= num,
                    _ => continue,
                }

                if c == '+' || c == '-' || idx == (n - 1) as usize {
                    res += cur_res;
                    cur_res = 0;
                }

                op = c;
                num = 0;
            }
        }

        res

    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_calculate() {
        assert_eq!(Solution::calculate("3+2*2".to_string()), 7);
        assert_eq!(Solution::calculate(" 3/2".to_string()), 1);
        assert_eq!(Solution::calculate(" 3+5 / 2".to_string()), 5);
    }
}