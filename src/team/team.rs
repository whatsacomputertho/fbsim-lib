use crate::team::player::Player;
use crate::team::units::offense::offense::Offense;
use crate::team::units::defense::defense::Defense;
use crate::team::units::specialteams::specialteams::SpecialTeams;
use crate::game::coinflip::CoinFlipDecision;

use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Bernoulli};

/// # Team struct
///
/// A `Team` represents a team of football players
pub struct Team {
    pub offense: Offense,
    pub defense: Defense,
    pub special_teams: SpecialTeams,
    pub name: String,
    pub abbreviation: String
}

impl Team {
    /// Initialize a new team given a Vec of Players and its
    /// name as a &str
    pub fn new(name: &str, abbreviation: &str) -> Team {
        Team {
            offense: Offense::new(),
            defense: Defense::new(),
            special_teams: SpecialTeams::new(),
            name: String::from(name),
            abbreviation: String::from(abbreviation)
        }
    }

    /// Calculate the team's overall rating
    pub fn get_overall(&self) -> f64 {
        // Calculate the sum of player overalls on the offense
        let offense_num_players: usize = self.offense.get_num_players();
        let offense_overall: f64 = self.offense.get_overall();
        let offense_total_overall: f64 = offense_overall * (offense_num_players as f64);

        // Calculate the sum of player overalls on the defense
        let defense_num_players: usize = self.defense.get_num_players();
        let defense_overall: f64 = self.defense.get_overall();
        let defense_total_overall: f64 = defense_overall * (defense_num_players as f64);

        // Calculate the sum of player overalls on the special teams
        let special_teams_num_players: usize = self.special_teams.get_num_players();
        let special_teams_overall: f64 = self.special_teams.get_overall();
        let special_teams_total_overall: f64 = special_teams_overall * (special_teams_num_players as f64);

        // Calculate and return the overall
        let num_players: usize = offense_num_players + defense_num_players + special_teams_num_players;
        let defense_total_overall: f64 = offense_total_overall + defense_total_overall + special_teams_total_overall;
        defense_total_overall / (num_players as f64)
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