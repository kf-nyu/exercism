use std::cmp::max;

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let Some((first, rest)) = items.split_first() else {
        return 0;
    };
    if first.weight > max_weight {
        maximum_value(max_weight, rest)
    } else {
        max(
            maximum_value(max_weight, rest),
            first.value + maximum_value(max_weight - first.weight, rest),
        )
    }
}
