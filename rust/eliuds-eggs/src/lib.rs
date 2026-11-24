pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 {
        return 0usize;
    }

    let digits: u32 = display_value.ilog2();

    let mut sol: usize = 0;
    let mut rem = display_value;

    for i in 0..=digits {
        if rem / 2_u32.pow(digits - i) > 0 {
            if rem.is_multiple_of(2_u32.pow(digits - i)) {
                return sol + 1;
            } else {
                sol += 1;
                rem -= 2_u32.pow(digits - i)
            }
        }
    }
    sol
}
