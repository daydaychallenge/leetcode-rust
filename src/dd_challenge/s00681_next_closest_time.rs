use crate::dd_challenge::Solution;

impl Solution {
    pub fn next_closest_time(time: String) -> String {
        let time_digits: Vec<char> = time.chars().filter(|x| *x != ':').collect();
        let to_usize = |x| time_digits[x] as usize - 48;
        let base_value = 60 * (10 * to_usize(0) + to_usize(1)) + 10 * to_usize(2) + to_usize(3);

        let mut letters = vec![false; 10];

        for d in time_digits.iter() {
            let i = *d as usize - 48;
            letters[i] = true;
        }

        let numbers: Vec<bool> = (0..60).map(|i| letters[i % 10] && letters[i / 10]).collect();
        let mut current_value = (base_value + 1) % 2400;

        loop {
            let (first, second) = (current_value / 60, current_value % 60);
            if first < 24 && second < 60 && numbers[first] && numbers[second] {
                return format!("{:02}:{:02}", first, second);
            }

            current_value = (current_value + 1) % 2400;
        }

    }
}

#[test]
fn test_next_closest_time() {
    assert_eq!(Solution::next_closest_time("19:34".to_string()), "19:39".to_string());
    assert_eq!(Solution::next_closest_time("23:59".to_string()), "22:22".to_string());
}