#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    fn factor(num: u64) -> u64 {
        (1..=num / 2).filter(|&n| num.is_multiple_of(n)).sum()
    }

    match factor(num) {
        x if x > num => Some(Classification::Abundant),
        x if x == num => {
            if num == 0 {
                None
            } else {
                Some(Classification::Perfect)
            }
        }
        _ => Some(Classification::Deficient),
    }
}
