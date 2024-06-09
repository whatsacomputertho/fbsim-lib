use std::fmt;

/// # GameScore struct
///
/// A `GameScore` represents the score of a football game
pub struct GameScore {
    pub home_team_score: usize,
    pub away_team_score: usize
}

impl GameScore {
    /// Initialize a zeroed GameScore
    pub fn new() -> GameScore {
        GameScore {
            home_team_score: 0_usize,
            away_team_score: 0_usize
        }
    }
}

impl fmt::Display for GameScore {
    /// Format a `GameScore` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let score_str = format!("{} - {}", self.home_team_score, self.away_team_score);
        f.write_str(&score_str)
    }
}