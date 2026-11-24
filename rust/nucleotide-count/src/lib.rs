use std::collections::HashMap;
const NUC: [char; 4] = ['A', 'C', 'G', 'T'];

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    //let cc = 0;

    if !NUC.contains(&nucleotide) {
        return Err(nucleotide);
    };

    match dna.chars().find(|c| !NUC.contains(c)) {
        Some(c) => Err(c),
        None => Ok(dna.chars().filter(|&c| c == nucleotide).count()),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut sol = HashMap::new();
    for c in NUC {
        let i = count(c, dna)?;
        sol.insert(c, i);
    }
    Ok(sol)
}
