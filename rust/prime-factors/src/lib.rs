pub fn factors(n: u64) -> Vec<u64> {
    let mut factors:Vec<u64> = Vec::new();
    let mut q:u64 = n;
    let mut i:u64 = 1;
    loop {
        i += 1;
            loop {
                if q.is_multiple_of(i) {
                    factors.push(i);
                    q /= i
                } else { break }
            }
        if q == 1 { break }
        }
    factors
}
