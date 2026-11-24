use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    number: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn new(n: u64) -> Self {
        Self {
            number: n,
            factors: HashSet::new(),
        }
    }
    pub fn value(&self) -> u64 {
        self.number
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut min_pal = Palindrome::new(0);
    let mut max_pal = Palindrome::new(0);
    for x in min..=max {
        for y in x..=max {
            let xy = x * y;
            let digits: String = xy.to_string();
            let reversed: String = digits.chars().rev().collect();
            if digits == reversed {
                if min_pal.number == 0 || xy < min_pal.number {
                    min_pal.number = xy;
                    min_pal.factors.clear();
                    min_pal.factors.insert((x, y));
                } else if min_pal.number == xy {
                    min_pal.factors.insert((x, y));
                };
                if max_pal.number < xy {
                    max_pal.number = xy;
                    max_pal.factors.clear();
                    max_pal.factors.insert((x, y));
                } else if max_pal.number == xy {
                    max_pal.factors.insert((x, y));
                };
            };
        }
    }

    match min_pal.value() {
        0 => None,
        _ => Some((min_pal, max_pal)),
    }
}
