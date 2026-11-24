use std::collections::HashSet;

fn sorted_chars(word: &str) -> Vec<char> {
    let mut word_vec:Vec<_> = word.chars().collect();
    word_vec.sort_unstable();
    word_vec
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    
    let word_s = word.to_lowercase();
    let critirion:Vec<char> = sorted_chars(&word_s);
    // we don't neend this
    //let mut list:HashSet<&'a str> = HashSet::new();
    
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_s = candidate.to_lowercase();
            candidate_s != word_s && sorted_chars(&candidate_s) == critirion
        })
        .copied()
        .collect()
}
