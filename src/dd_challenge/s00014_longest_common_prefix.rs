use crate::dd_challenge::Solution;

///
/// 求出数组中最短字符串的长度min_len
/// 求出第一个元素不大于最短长度min_len字符串的前缀字符串数组
/// 遍历所有前缀中的元素, 逐一判断是否所有元素都以此前缀开始, 如果不是,返回上一个前缀元素或""
///
impl Solution {
    pub fn longest_common_prefix(ss: Vec<String>) -> String {
        let mix_len: usize = ss.iter().map(|s| s.chars().collect::<Vec<char>>().len()).min().unwrap();

        let prefixes = Solution::prefix(ss[0].to_owned(), mix_len);

        println!("The all prefix is: {:?}", prefixes);

        for (idx, prefix) in prefixes.iter().enumerate() {
            for s in &ss {
                println!("The prefix: {:?}", prefix);
                if !s.starts_with(prefix) {
                    return if idx > 1 { prefixes[idx - 1].to_owned() } else { "".to_string() }
                } else {

                }
            }
        }

        prefixes.last().unwrap().to_owned()


    }

    fn prefix(s: String, size: usize) -> Vec<String> {
        let mut res = Vec::new();
        for idx in 1..size + 1 {
            res.push(s.chars().take(idx).collect::<String>());
        }

        res
    }

    fn min_len(ss: &Vec<String>) -> usize {
        ss.iter().map(|s| s.chars().collect::<Vec<char>>().len()).min().unwrap()
    }
}

pub fn do_sth() {

    let ss = vec!["abcbed".to_string(), "ab".to_string(), "abka".to_string()];
    let min_len = Solution::min_len(&ss);
    let prefix = Solution::longest_common_prefix(ss);
    println!("min len: {:?}", min_len);
    println!("{:?}", prefix);
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_longest_common_prefix() {
        let ss1 = vec!["flower","flow","flight"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::longest_common_prefix(ss1), "fl".to_string());

        let ss2 = vec!["dog","racecar","car"].iter().map(|x| x.to_string()).collect();
        assert_eq!(Solution::longest_common_prefix(ss2), "".to_string());
    }
}