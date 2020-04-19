
pub fn do_sth() {
    let mut s = vec!['a', 'b', 'c', 'd'];
    let len = s.len();
    let r = reverse(&mut s, 0, len);
    println!("after reversed: {:?}", r);
}

pub fn reverse_words(s: &mut Vec<char>) {
    if s.is_empty() {
        return;
    }

    s.reverse();

    let mut s_len = s.len();
    let mut start = 0;

    while start < s_len {
        let mut end = start;
        while end + 1 < s_len && s[end + 1] != ' ' {
            end += 1;
        }

        let new_idx = if end + 1 >= s_len {
            end + 1
        } else {
            end + 2
        };

        reverse(s, start, end);

        start = new_idx;
    }

}

fn reverse(s: &mut Vec<char>, left: usize, right: usize) {

    let mut start = left;
    let mut end = right;
    while start < end {
        s.swap(start, end);
        start += 1;
        end -= 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reverse_words() {
        let mut src = vec!['t', 'h', 'e', ' ', 's', 'k', 'y', ' ', 'i', 's', ' ', 'b', 'l', 'u', 'e'];
        let tar = vec!['b', 'l', 'u', 'e', ' ', 'i', 's', ' ', 's', 'k', 'y', ' ', 't', 'h', 'e'];
        super::reverse_words(&mut src);
        assert_eq!(src, tar);
    }
}