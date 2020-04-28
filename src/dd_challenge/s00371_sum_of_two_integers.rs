use crate::dd_challenge::Solution;

///
/// 371. Sum of Two Integers
/// Calculate the sum of two integers a and b, but you are not allowed to use the operator + and -.
///
/// Example 1:
///
/// Input: a = 1, b = 2
/// Output: 3
/// Example 2:
///
/// Input: a = -2, b = 3
/// Output: 1
///
/// 解题思路
/// 首先明白异或和与运算
/// a^b 异或是无进位累加的和 比如 01^01=00 即不会有进位 正常进位为10 说白了异或就是砍掉进位
/// a&b 后的结果左移以为可以获取进位值 比如 01&01=01 01 << 1 = 10 与运算后左移以为拿到进位的值 10
///
/// 那么此时 (a^b) + (a&b)<< 1 即累加后的结果,原理即无进位的值+进位的值等于一个数
///
/// 第二步考虑什么时候计算完毕,即进位数位0时,如下代码b=0时 a:代表无进位累加值 b:代表进位值
///
/// 比如 12+7
///
/// 12 1 1 0 0
/// 7  0 1 1 1
///
/// 12^7 1 0 1 1 无进位累加值为11
/// 12&7 0 1 0 0 << 1 = 1 0 0 0 进位值为8
///
/// 11 1 0 1 1
/// 8  1 0 0 0
///
/// 11^8 0 0 1 1 无进位累加值为3
/// 11&8 1 0 0 0 << 1 = 1 0 0 0 0 进位值为16
///
/// 3  0 0 0 1 1
/// 16 1 0 0 0 0
///
/// 3^16 1 0 0 1 1 无进位累加值为19
/// 3&16 0 0 0 0 0 << 1 = 0 0 0 0 0 0 进位值为0
///
/// 当进位为0时即退出即结果为无进位累加值19
///
///
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::get_sum(a ^ b, (a & b) << 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_get_sum() {
        let a1 = 1;
        let b1 = 2;
        let res1 = Solution::get_sum(a1, b1);
        assert_eq!(res1, 3);

        let (a2, b2) = (-2, 3);
        assert_eq!(Solution::get_sum(a2, b2), 1);
    }
}