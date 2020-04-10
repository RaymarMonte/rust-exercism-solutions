#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {scores: scores.to_vec()}
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            Some(self.scores[self.scores.len()-1])
        }
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            None
        } else {
            let mut sorted = self.scores.to_vec();
            sorted.sort();
            Some(sorted[sorted.len()-1])
        }
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut sorted = self.scores.to_vec();
        sorted.sort();
        sorted.iter().rev().take(3).cloned().collect()
    }
}
