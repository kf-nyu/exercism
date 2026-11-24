pub struct Luhn {
    val: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let clensed: String = self.val.chars().filter(|sp| !sp.is_whitespace()).collect();

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

impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Self {
            val: input.to_string(),
        }
    }
}
