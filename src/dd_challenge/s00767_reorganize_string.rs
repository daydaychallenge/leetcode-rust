use crate::dd_challenge::Solution;
use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let mut heap: BinaryHeap<CharFrequency> = create_heap(&s);
        let mut result: String = String::new();

        while !heap.is_empty() {
            let top = heap.pop().unwrap();
            result.push(top.chr);
            if let Some(other_top) = heap.pop() {
                result.push(other_top.chr);

                if other_top.frequency > 1 {
                    heap.push(CharFrequency::new(other_top.chr, other_top.frequency - 1));
                }
            } else if top.frequency > 1 {
                return String::new();
            }

            if top.frequency > 1 {
                heap.push(CharFrequency::new(top.chr, top.frequency - 1));
            }
        }

        result
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct CharFrequency {
    frequency: usize,
    chr: char,
}

impl CharFrequency {
    fn new(chr: char, frequency: usize) -> Self {
        CharFrequency{ frequency, chr }
    }
}

fn create_heap(s: &str) -> BinaryHeap<CharFrequency> {
    s.chars()
        .fold(HashMap::new(), |mut map, chr| {
            *map.entry(chr).or_insert(0) += 1usize;
            map
        })
        .into_iter()
        .map(|(k, v)| CharFrequency::new(k, v))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::dd_challenge::Solution;

    #[test]
    fn test_reorganize_string() {
        assert_eq!(Solution::reorganize_string("aab".to_string()), "aba".to_string());
        assert_eq!(Solution::reorganize_string("aaab".to_string()), "".to_string());
    }
}