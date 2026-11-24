#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(scores.to_vec())
    }

    pub fn scores(&self) -> &[u32] {
        &self.0
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.iter().copied().last()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0.iter().copied().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut temp = self.0.clone();
        temp.sort_unstable_by(|a, b| b.cmp(a));
        temp.truncate(3);
        temp
    }
}
