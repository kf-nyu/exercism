pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() {
        return Vec::from([]);
    }

    let v: Vec<String> = digits.chars().map(|c| c.to_string()).collect();
    let mut sol = Vec::new();

    for window in v.windows(len) {
        sol.push(window.concat())
    }

    sol
}
