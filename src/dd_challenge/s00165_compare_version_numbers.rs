use crate::dd_challenge::Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut v1: Vec<i32> = version1.split('.').map(|v| v.parse().unwrap_or(0)).collect();
        let mut v2: Vec<i32> = version2.split('.').map(|v| v.parse().unwrap_or(0)).collect();

        let gap = if v1.len() > v2.len() { v1.len() - v2.len() } else { v2.len() - v1.len() };

        let mut flag = 0;
        while flag < gap {
            if v1.len() < v2.len() { v1.push(0); }
            else  {
                v2.push(0);
            }
            flag +=1;
        }

        for (c1, c2) in v1.into_iter().zip(v2.into_iter()) {
            if c1 < c2 {
                return -1;
            } else if c1 > c2 {
                return 1;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn test_compare_version() {
        assert_eq!(Solution::compare_version("0.1".to_string(), "1.1".to_string()), -1);
        assert_eq!(Solution::compare_version("1.0.1".to_string(), "1.".to_string()), 1);
        assert_eq!(Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string()), -1);
    }
}

pub fn do_sth() {
    let s = "1".to_string();
    let v: Vec<i32> = s.split('.').map(|v| v.parse().unwrap_or(0)).collect();
    println!("{:?}", v);

    Solution::compare_version("1.0.1".to_string(), "1".to_string());
}