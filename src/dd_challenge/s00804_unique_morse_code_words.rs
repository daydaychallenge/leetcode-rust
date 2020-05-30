use crate::dd_challenge::Solution;

const MORSE_MAP: [&str; 26] = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..",
    ".---", "-.-", ".-..", "--","-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-",
    ".--", "-..-", "-.--", "--.."];

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let s = words.into_iter()
            .map(|word| {
                word.into_bytes()
                    .into_iter()
                    .map(|c| {
                        String::from(MORSE_MAP[(c - 'a' as u8) as usize])
                    })
                    .collect()
            })
            .collect::<std::collections::BTreeSet<String>>();
        println!("{:?}", s);
        s.len() as i32
    }
}


#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_unique_morse_representations() {
        let words = vec!["gin".to_string(), "zen".to_string(), "gig".to_string(), "msg".to_string()];
        assert_eq!(Solution::unique_morse_representations(words), 2);
    }
}