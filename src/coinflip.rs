use crate::team::Team;

use rand::Rng;
use rand::distributions::{Distribution, Bernoulli};

/// # CoinFlipDecision enum
///
/// A `CoinFlipDecision` represents the possible decisions that
/// a team can make upon winning the opening coin flip.
pub enum CoinFlipDecision {
    KICK,
    RECEIVE
}

/// # CoinFlip struct
///
/// A `CoinFlip` represents a coin flip at the beginning of a
/// football game to determine who receives the opening kickoff,
/// and who receives the halftime kickoff.
pub struct CoinFlip {
    pub heads: bool,
    pub guess_heads: bool,
    pub decision: CoinFlipDecision
}

impl CoinFlip {
    /// Initialize a CoinFlip struct instance with default values
    pub fn new() -> CoinFlip {
        CoinFlip {
            heads: false,
            guess_heads: false,
            decision: CoinFlipDecision::KICK
        }
    }

    /// Check whether the away team won the coin flip
    pub fn away_team_won(&self) -> bool {
        (self.heads && self.guess_heads) || !(self.heads || self.guess_heads)
    }

    /// Allow the away team to make a guess on the result of the
    /// coin flip.  Flip the coin, and then get the winning team's
    /// decision on who should receive the opening kickoff, and who
    /// should receive the halftime kickoff.
    pub fn simulate(&mut self, away_team: &Team, home_team: &Team, mut rng: &mut impl Rng) {
        // Get the away team's guess
        self.guess_heads = away_team.get_coinflip_guess(&mut rng);

        // Flip the coin
        let dst_flip: Bernoulli = Bernoulli::new(0.5_f64).unwrap();
        self.heads = dst_flip.sample(&mut rng);

        // Get the winning team's decision
        self.decision = match self.away_team_won() {
            true => away_team.get_coinflip_decision(&mut rng),
            false => home_team.get_coinflip_decision(&mut rng)
        };
    }
}