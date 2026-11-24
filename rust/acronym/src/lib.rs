pub fn abbreviate(phrase: &str) -> String {
    
    let mut t = String::new();
    let mut prev = ' ';
    for c in phrase.chars() {
        if c.is_uppercase() && prev.is_alphabetic() && prev.is_lowercase()  {
            t.push(' ')
        }
        t.push(c);
        prev = c;
    }
    t.split([' ','-','_'])
        .collect::<Vec<_>>()
        .iter()
        .copied()
        .filter(|&c| !c.is_empty())
        .map(|word| word.chars().next().unwrap())
        .collect::<String>()
        .to_uppercase()
}
