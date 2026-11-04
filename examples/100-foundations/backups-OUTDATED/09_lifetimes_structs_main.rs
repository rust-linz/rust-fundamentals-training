#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { scores }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut v = self.scores.to_vec();
        v.sort();
        v.iter().rev().take(3).copied().collect()
    }
}

fn main() {
    let scores = [10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70];
    let high_scores = HighScores::new(&scores);

    println!("{:?}", high_scores.personal_top_three());
}
