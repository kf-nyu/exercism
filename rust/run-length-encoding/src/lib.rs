pub fn encode(source: &str) -> String {
    if source.len() <= 1 {
        return source.to_string();
    };
    let mut encoded = String::new();
    let mut char_read = ' ';
    let mut char_count = 0;
    for c in source.chars() {
        if c == char_read {
            char_count += 1
        } else {
            if char_count > 1 {
                encoded.push_str(&format!("{}{}", char_count, char_read))
            } else {
                if char_count == 1 {
                    encoded.push(char_read)
                };
            }
            char_count = 1;
            char_read = c
        }
    }
    if char_count > 1 {
        encoded.push_str(&format!("{}{}", char_count, char_read))
    } else {
        encoded.push(char_read)
    }
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut char_count = 0;
    for c in source.chars() {
        if c.is_ascii_digit() {
            char_count *= 10;
            char_count += c.to_digit(10).unwrap()
        } else {
            if char_count == 0 {
                decoded.push(c)
            } else {
                decoded.push_str(&c.to_string().repeat(char_count as usize))
            }
            char_count = 0;
        }
    }
    decoded
}
