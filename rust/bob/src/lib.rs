pub fn reply(message: &str) -> &str {
    let trimmed: String = message
        .trim_end()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '?')
        .collect();

    if trimmed.ends_with('?') {
        let trimmed_s: String = trimmed.chars().filter(|c| c.is_alphabetic()).collect();
        if trimmed_s.chars().all(|x| x.is_uppercase()) && !trimmed_s.is_empty() {
            return "Calm down, I know what I'm doing!";
        } else {
            return "Sure.";
        }
    } else {
        if trimmed.is_empty() {
            return "Fine. Be that way!";
        } else {
            let trimmed_s: String = trimmed.chars().filter(|c| c.is_alphabetic()).collect();
            if trimmed_s.chars().all(|x| x.is_uppercase()) && !trimmed_s.is_empty() {
                return "Whoa, chill out!";
            }
        }
    }
    "Whatever."
}
