
pub fn longest_valid_parentheses(s: String) -> i32 {
    let mut seq: Vec<char> = s.chars().collect();
    let forward_max = longest(&seq, '(');

    seq.reverse();
    let backward_max = longest(&seq, ')');
    i32::max(forward_max, backward_max)
}

fn longest(seq: &Vec<char>, plus_char: char) -> i32 {
    let mut stack = 0;
    let mut max_len = 0;

    let (mut i, mut j) = (0_usize, 0_usize);

    while j < seq.len() {
        if seq[j] == plus_char {
            stack += 1;
        } else {
            //
            if stack < 1 {
                i = j + 1;
            } else {
                stack -= 1;
                if stack == 0 {
                    max_len = i32::max(max_len, (j - i + 1) as i32);
                }
            }
        }
        j += 1;
    }

    max_len
}