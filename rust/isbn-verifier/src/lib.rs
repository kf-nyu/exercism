/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let arr: Vec<_> = isbn.chars().filter(|&a| a != '-').collect();
    if arr.len() != 10 {
        return false;
    };
    let mut arr_sum = 0;
    for (i, &c) in arr.iter().enumerate() {
        if c.is_numeric() {
            arr_sum += (10 - i as u32) * c.to_digit(10).unwrap()
        } else {
            if i == 9 && c == 'X' {
                arr_sum += 10
            } else {
                return false;
            }
        }
    }
    arr_sum % 11 == 0
}
