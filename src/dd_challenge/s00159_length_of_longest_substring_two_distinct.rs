use crate::dd_challenge::Solution;
use std::cmp::max;

impl Solution {
    pub fn length_of_longest_substring_two_distinct(s: String) -> i32 {
        let k: usize = 2usize;
        let len = s.len();
        if k >= len {
            return len as i32;
        }

        let ss = s.as_bytes();
        let mut res = 0;
        let mp = &mut [0i32; 256];
        let (mut l, mut r) = (0usize, 1usize);
        let mut count = 0usize;
        while r <= len {
            let us = ss[r - 1] as usize;
            mp[us] += 1;
            if mp[us] == 1 {
                count += 1;

                while l < r - 1 && count > k {
                    let ps = ss[l] as usize;
                    mp[ps] -= 1;
                    if mp[ps] == 0 {
                        count -= 1;
                    }

                    l += 1;
                }
            }

            res = max(res, r - l);
            r += 1;
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_length_of_longest_substring_k_distinct() {
        assert_eq!(Solution::length_of_longest_substring_two_distinct("eceba".to_string()), 3);
        assert_eq!(Solution::length_of_longest_substring_two_distinct("aa".to_string()), 2);
        assert_eq!(Solution::length_of_longest_substring_two_distinct("ccaabbb".to_string()), 5);
    }
}