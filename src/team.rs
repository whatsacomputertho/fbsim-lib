use crate::player::Player;
use crate::coinflip::CoinFlipDecision;

use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Bernoulli};

/// # Team struct
///
/// A `Team` represents a team of football players
pub struct Team {
    pub players: Vec<Player>,
    pub name: String,
    pub abbreviation: String
}

impl Team {
    /// Initialize a new team given a Vec of Players and its
    /// name as a &str
    pub fn new(players: Vec<Player>, name: &str, abbreviation: &str) -> Team {
        Team {
            players: players,
            name: String::from(name),
            abbreviation: String::from(abbreviation)
        }
    }

    /// Calculate the team's overall rating
    pub fn get_overall(&self) -> f64 {
        let num_players: f64 = self.players.len() as f64;
        let mut sum_player_overall: f64 = 0.0_f64;
        for player in self.players.iter() {
            sum_player_overall += player.get_overall();
        }
        sum_player_overall / num_players
    }

    /// Choose whether to guess heads or tails going into the
    /// opening coin flip
    pub fn get_coinflip_guess(&self, mut rng: &mut impl Rng) -> bool {
        let dst_guess_heads: Bernoulli = Bernoulli::new(0.5_f64).unwrap();
        let guess_heads: bool = dst_guess_heads.sample(&mut rng);
        guess_heads
    }

    /// Choose whether to kick (defer), or receive upon winning
    /// the opening coin flip
    pub fn get_coinflip_decision(&self, mut rng: &mut impl Rng) -> CoinFlipDecision {
        let dst_decision: Bernoulli = Bernoulli::new(0.5_f64).unwrap();
        let decide_kick: bool = dst_decision.sample(&mut rng);
        let decision: CoinFlipDecision = match decide_kick {
            true => CoinFlipDecision::KICK,
            false => CoinFlipDecision::RECEIVE
        };
        decision
    }
}

impl fmt::Display for Team {
    /// Format a `Team` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let team_str = format!("{} ({:.2} Overall)", self.name, self.get_overall());
        f.write_str(&team_str)
    }
}