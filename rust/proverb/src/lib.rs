pub fn build_proverb(list: &[&str]) -> String {
    let mut output = String::new();
    for window in list.windows(2) {
        output.push_str(&format!("For want of a {} the {} was lost.\n", window[0], window[1]))
    }
    if !list.is_empty() {
        output.push_str(&format!("And all for the want of a {}.", list[0]))
    }
    output
}
