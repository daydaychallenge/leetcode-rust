use crate::dd_challenge::Solution;
use crate::dd_challenge::s00273_integer_to_english_words::TreeNode::{Zero, OneToTwenty, TwentyPlus};

/**
思路
我们要把数字变成英语可以表达的, 就只能用英语里面有的词, 英语里面有哪些词可以表达数字?
能翻译成单词的数字只有

0
1~19
20, 30, ... , 90
D(100), K(1,000), M(1,000,000), G(1,000,000,000)
所以我们要把一个整数分解到这些数字上, 形成数字->单词的映射. (这样做也规避了空格的问题, 毕竟一个单词一个空格嘛...)
通过一个整数n, 向下生成子树
最终使得每一个叶子节点, 都可以根据它携带的值, 翻译成对应的单词, 如果叶子节点标0, 则翻译成空字符串.
举个例子, 比如有
let n=114_514;
匹配到:1_000 < n < 1_000_000

(114_514)
｜
ｏ－－－－－－－－＼
｜　　　　　　　　｜
ｏ－－－－＼　　　｜
｜　　　　｜　　　｜
１１４　　Ｋ　　５１４
我们根据这个整数生成了如下子树, 除了K是叶子节点以外, 114, 514 都是100以上的整数, 还可以继续往下生成.
以114为例, 又匹配到100 < 114 < 1_000

(114)
｜
ｏ－－－－－－－－＼
｜　　　　　　　　｜
ｏ－－－－＼　　　｜
｜　　　　｜　　　｜
１　　　　Ｄ　　　１４
到这里, 1, D, 14 都是可以接受的叶子了.
整个树会像如下

(114_514)
｜
ｏ－－－－－－－－－－＼
｜　　　　　　　　　　　｜
ｏ－－－－－－－＼　　　｜
｜　　　　　　　　｜　　｜
｜　　　　　　　　｜　　｜
｜　　　　　　　　｜　　｜
ｏ－－＼　　　　　｜　　ｏ－－＼
｜　　　｜　　　　｜　　｜　　　｜
ｏ＼　　｜　　　　｜　　ｏ＼　　｜
｜　｜　｜　　　　｜　　｜　｜　｜
１　Ｄ　１４　　　Ｋ　　５　Ｄ　１４
对于一定大小范围内的数字, 它的分解方法是确定的, 我们只需要写出不同范围内数字的分解方法, 然后再翻译一下就好了
翻译也很简单, 对于每一个子树, 它对应的词就是左右子树拼在一起, 层层递归就行.

Code
执行结果0ms, 2mb
实际上可以把翻译的时候, match一大段写成数组形式
但是我觉得写成match比较直观, 就这么写了

*/

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 { "Zero".to_string() }
        else {
            gen(num).to_string().trim_end().to_owned()
        }
    }

}

const G: i32 = 1_000_000_000;
const M: i32 = 1_000_000;
const K: i32 = 1_000;
const D: i32 = 100;
const C: i32 = 10;

use TreeNode::*;

fn gen(n: i32) -> Box<TreeNode> {
    match n {
        0 => { Box::new(Zero) },
        n if n < 20 => { Box::new(OneToTwenty(n)) },
        n if n < D => {
            Box::new(
                Cons(Box::new(TwentyPlus(n / C)), Box::new(OneToTwenty(n % C)))
            )
        },
        n if n < K => {
            Box::new(Cons(
                Box::new(Cons(
                    Box::new(OneToTwenty(n / D)),
                    Box::new(Hundred(n / D))
                )),
                gen(n % D)
            ))
        },
         n if n < M => {
            Box::new(Cons(
                Box::new(Cons(
                    gen(n / K),
                    Box::new(Thousand(n / K))
                )),
                gen( n % K)
            ))
         },
        n if n < G => {
            Box::new(Cons(
                Box::new(Cons(
                    gen( n / M),
                    Box::new(Million(n / M))
                )),
                gen(n % M)
            ))
        },
        n if n <= i32::max_value() => {
            Box::new(Cons(
                Box::new(Cons(
                    gen(n / G),
                    Box::new(Billion(n / G))
                )),
                gen(n % G)
            ))
        },
        _ => Box::new(Zero)
    }
}

enum TreeNode {
    Cons(Box<TreeNode>, Box<TreeNode>),

    Billion(i32),
    Million(i32),
    Thousand(i32),
    Hundred(i32),
    TwentyPlus(i32),
    OneToTwenty(i32),
    Zero
}

impl TreeNode {
    fn to_string(&self) -> String {
        match self {
            Cons(lhs,rhs) => {
                format!("{}{}",
                        lhs.to_string(),
                        rhs.to_string(),
                )},
            Billion(x) => {( if *x==0 {""} else {"Billion "} ).to_owned()},
            Million(x) => {( if *x==0 {""} else {"Million "} ).to_owned()},
            Thousand(x) => {( if *x==0 {""} else {"Thousand "} ).to_owned()},
            Hundred(x) => {( if *x==0 {""} else {"Hundred "} ).to_owned()},
            TwentyPlus(x) => {
                (match *x {
                    2 => "Twenty ",
                    3 => "Thirty ",
                    4 => "Forty ",
                    5 => "Fifty ",
                    6 => "Sixty ",
                    7 => "Seventy ",
                    8 => "Eighty ",
                    9 => "Ninety ",
                    _ => "",
                }).to_owned()
            },
            OneToTwenty(x) => {
                (match *x {
                    1 => "One ",
                    2 => "Two ",
                    3 => "Three ",
                    4 => "Four ",
                    5 => "Five ",
                    6 => "Six ",
                    7 => "Seven ",
                    8 => "Eight ",
                    9 => "Nine ",
                    10 => "Ten ",
                    11 => "Eleven ",
                    12 => "Twelve ",
                    13 => "Thirteen ",
                    14 => "Fourteen ",
                    15 => "Fifteen ",
                    16 => "Sixteen ",
                    17 => "Seventeen ",
                    18 => "Eighteen ",
                    19 => "Nineteen ",
                    _ => "",
                }).to_owned()
            },
            Zero => {"".to_owned()}
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_number_to_words() {
        let s1 = 123;
        assert_eq!(Solution::number_to_words(s1), "One Hundred Twenty Three");

        let s2 = 12345;
        assert_eq!(Solution::number_to_words(s2), "Twelve Thousand Three Hundred Forty Five");

        let s3 = 1234567;
        assert_eq!(Solution::number_to_words(s3), "One Million Two Hundred Thirty Four Thousand Five Hundred Sixty Seven");

        let s4 = 1234567891;
        assert_eq!(Solution::number_to_words(s4),
                   "One Billion Two Hundred Thirty Four Million Five Hundred Sixty Seven Thousand Eight Hundred Ninety One"
        );
    }
}