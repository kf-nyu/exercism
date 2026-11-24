pub fn square(s: u32) -> u64 {
    if s > 64 {
        panic!("error")
    } else {
        2_u64.pow(s - 1) as u64
    }
}

pub fn total() -> u64 {
    (1..=64).map(|i| square(i)).sum::<u64>()
}
