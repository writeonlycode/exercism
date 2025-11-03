#[derive(Debug)]
pub struct HighScores(Vec<u32>);

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores(scores.to_vec())
    }

    pub fn scores(&self) -> &[u32] {
        self.0.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.0.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.0
            .iter()
            .reduce(|acc, e| if acc < e { e } else { acc })
            .copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut a = self.0.clone();

        a.sort();
        a.reverse();

        match a.len() {
            0 => Vec::new(),
            1 => a.first_chunk::<1>().unwrap().to_vec(),
            2 => a.first_chunk::<2>().unwrap().to_vec(),
            _ => a.first_chunk::<3>().unwrap().to_vec(),
        }
    }
}
