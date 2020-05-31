use crate::dd_challenge::Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let n = s.len();
        let mut f = vec![0i32; n + 1];
        let raw = s.as_bytes();

        f[0] = 1i32;
        f[1] = if raw[0] == b'0' { 0 } else { 1i32 };

        for l in 2..=n {
            let idx = l - 1;
            f[l] = if raw[idx] != b'0' { f[l - 1] } else { 0 } +
                ({
                    let sum = raw[idx] - b'0' + (raw[idx - 1] - b'0') * 10;
                    if sum <= 26 && raw[idx - 1] != b'0' {
                        f[l - 2]
                    } else {
                        0
                    }
                })
        };

        f[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_num_decodings() {
        let input = "12".to_string();
        assert_eq!(Solution::num_decodings(input), 2);
        assert_eq!(Solution::num_decodings("226".to_string()), 3);
    }
}