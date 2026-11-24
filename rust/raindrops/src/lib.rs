pub fn raindrops(n: u32) -> String {

    let mut output = String::new();

    if n.is_multiple_of(3) {output.push_str("Pling")};
    if n.is_multiple_of(5) {output.push_str("Plang")};
    if n.is_multiple_of(7) {output.push_str("Plong")};
    if output.is_empty() {output.push_str(&n.to_string()) };

    output

}
