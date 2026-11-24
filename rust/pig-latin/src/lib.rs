pub fn translate(input: &str) -> String {
    pub fn word_translate(input: &str) -> String {
        let vowels = ['a', 'i', 'u', 'e', 'o'];
        let xryt = ["xr", "yt"];

        let mut input_string: String = input.to_string();
        let two_chars: String = input.chars().take(2).collect();
        let two_str = &two_chars.as_str();
        if vowels.contains(&input.chars().next().unwrap()) || xryt.contains(two_str) {
            input_string.push_str("ay");
            return input_string;
        };
        if let Some(index) = input.find("qu")
            && !input[0..index].contains(vowels)
        {
            let mut qu_input: String = input[index + 2..].to_string();
            qu_input.push_str(&input[0..index + 2]);
            qu_input.push_str("ay");
            return qu_input;
        };
        if let Some(index) = input.find("y")
            && index != 0
            && !input[0..index].contains(vowels)
        {
            let mut y_input: String = input[index..].to_string();
            y_input.push_str(&input[0..index]);
            y_input.push_str("ay");
            return y_input;
        };
        let vowel_position = input.char_indices().find_map(|(index, c)| {
            if vowels.contains(&c) {
                Some(index)
            } else {
                None
            }
        });

        match vowel_position {
            Some(index) => {
                let mut con_input: String = input[index..].to_string();
                con_input.push_str(&input[0..index]);
                con_input.push_str("ay");
                con_input
            }
            None => {
                input_string.push_str("ay");
                input_string
            }
        }
    }

    input
        .split_whitespace()
        .map(word_translate)
        .collect::<Vec<String>>()
        .join(" ")
}
