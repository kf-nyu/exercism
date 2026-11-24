/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let clensed: String = code.chars().filter(|sp| !sp.is_whitespace()).collect();

    if clensed.len() < 2 || !clensed.chars().all(|c| c.is_numeric()) {
        return false;
    }

    let total: u32 = clensed
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .enumerate()
        .map(|(i, c)| {
            if i % 2 == 1 {
                if c > 4 { (c - 5) * 2 + 1 } else { 2 * c }
            } else {
                c
            }
        })
        .map(|b| b as u32)
        .sum();

    total % 10 == 0 
}
