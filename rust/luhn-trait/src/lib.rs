pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Here is the example of how to implement custom Luhn trait
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<T: ToString> Luhn for T {
    fn valid_luhn(&self) -> bool {
        let clensed: String = self
            .to_string()
            .chars()
            .filter(|sp| !sp.is_whitespace())
            .collect();

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

        total.is_multiple_of(10)
    }
}
