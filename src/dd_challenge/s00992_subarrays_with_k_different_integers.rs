use crate::dd_challenge::Solution;

impl Solution {
    pub fn subarrays_with_k_distinct(a: Vec<i32>, k: i32) -> i32 {
        Self::compute(&a, k) - Self::compute(&a, k - 1)
    }

    fn compute(a: &Vec<i32>, k: i32) -> i32 {
        let mut count = vec![0; a.len() + 1];

        let (mut i, mut ans, mut k) = (0, 0, k);

        for j in 0..a.len() {
            if count[a[j] as usize] == 0 {
                k -= 1;
            }
            count[a[j] as usize] += 1;

            while k < 0 {
                count[a[i] as usize] -= 1;
                i += 1;
                if count[a[i - 1] as usize] == 0 {
                    k += 1;
                }
            }

            ans += j - i + 1;
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_subarrays_with_k_distinct() {
        assert_eq!(super::Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
        assert_eq!(super::Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
    }
}