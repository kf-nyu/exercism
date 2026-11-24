pub fn is_armstrong_number(num: u32) -> bool {
    
    let digits = num.checked_ilog10().unwrap_or(0) + 1;
    
    num.to_string()
        .chars()
        .map(|c| { c.to_digit(10)
            .unwrap()
            .pow(digits)
            })
        .sum::<u32>() == num

}
