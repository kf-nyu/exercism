pub fn collatz(n: u64) -> Option<u64> {
    pub fn collatz_r(n: u64, tried: u64) -> Option<u64> {
        if n == 1 {
            return Some(tried);
        }

        if n.is_multiple_of(2) {
            collatz_r(n / 2, tried + 1)
        } else {
            collatz_r(n * 3 + 1, tried + 1)
        }
    }
    if n > 0 { collatz_r(n, 0) } else { None }
}
