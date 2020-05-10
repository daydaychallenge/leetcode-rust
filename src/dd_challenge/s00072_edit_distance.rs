use crate::dd_challenge::Solution;

//
// 72. Edit Distance
// Given two words word1 and word2, find the minimum number of operations required to convert word1 to word2.
//
// You have the following 3 operations permitted on a word:
//
// Insert a character
// Delete a character
// Replace a character
// Example 1:
//
// Input: word1 = "horse", word2 = "ros"
// Output: 3
// Explanation:
// horse -> rorse (replace 'h' with 'r')
// rorse -> rose (remove 'r')
// rose -> ros (remove 'e')
// Example 2:
//
// Input: word1 = "intention", word2 = "execution"
// Output: 5
// Explanation:
// intention -> inention (remove 't')
// inention -> enention (replace 'i' with 'e')
// enention -> exention (replace 'n' with 'x')
// exention -> exection (replace 'n' with 'c')
// exection -> execution (insert 'u')
//
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1_len, word2_len) = (word1.len(), word2.len());
        let (word1_bytes, word2_bytes) = (word1.into_bytes(), word2.into_bytes());

        if word1_len == 0 || word2_len == 0 {
            return word1_len.max(word2_len) as i32;
        }
        // dp[i][j] means how many operations required that covert word1[0..=i] to word2[0..=j]
        let mut dp = vec![vec![0; word2_len + 1]; word1_len + 1];

        // delete chars
        for i in 0..=word1_len {
            dp[i][0] = i as i32;
        }

        // insert chars
        for j in 0..=word2_len {
            dp[0][j] = j as i32;
        }

        for i in 1..=word1_len {
            for j in 1..=word2_len {
                // if equal, just forward because that it doesn't need any operation
                if word1_bytes[i - 1] == word2_bytes[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1]
                } else {
                    // any operation required(insert, delete, replace), so plus 1
                    // minimum number of operations is one of belows
                    // dp[i - 1][j]: delete one char
                    // dp[i][j - 1]: insert one char
                    // dp[i - 1][j - 1]: replace one char
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
            }
        }

        dp[word1_len][word2_len]
    }
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    fn test_min_distance() {
        assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
        assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
    }
}