#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

pub fn find_first_invalid_nucleotide(s: &str, nucleotides: &[char]) -> Option<(usize, char)> {
    s.char_indices()
        .find(|(_index, c)| !nucleotides.contains(c))
}

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        let dna_nucleotides = ['G', 'C', 'T', 'A'];
        match find_first_invalid_nucleotide(dna, &dna_nucleotides) {
            None => Ok(Self(dna.to_string())),
            Some((pos, _ch)) => Err(pos),
        }
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| match c {
                'G' => 'C',
                'C' => 'G',
                'T' => 'A',
                'A' => 'U',
                other => other,
            })
            .collect())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        let rna_nucleotides = ['C', 'G', 'A', 'U'];
        match find_first_invalid_nucleotide(rna, &rna_nucleotides) {
            None => Ok(Self(rna.to_string())),
            Some((pos, _ch)) => Err(pos),
        }
    }
}
