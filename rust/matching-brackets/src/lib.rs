pub fn brackets_are_balanced(string: &str) -> bool {
    fn is_valid_pair(open: char, close: char) -> bool {
        matches!((open, close), ('[', ']') | ('{', '}') | ('(', ')'))
    }

    let allowed: &[char] = &['[', ']', '{', '}', '(', ')'];
    let pop_allowed: &[char] = &[']', '}', ')'];

    let clean: Vec<char> = string.chars().filter(|c| allowed.contains(c)).collect();

    let mut queue = Vec::new();

    for item in &clean {
        if pop_allowed.contains(item) {
            if queue.is_empty() {
                return false;
            };
            if !is_valid_pair(queue.pop().expect("Type Error"), *item) {
                return false;
            }
        } else {
            queue.push(*item)
        }
    }

    queue.is_empty()
}
