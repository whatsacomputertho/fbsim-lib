use crate::team::Team;
use crate::score::GameScore;
use crate::clock::GameClock;
use crate::log::GameLog;
use crate::possession::GamePossession;
use crate::coinflip::{CoinFlip, CoinFlipDecision};

use std::fmt;
use rand::Rng;

/// # Game struct
///
/// The `Game` struct represents a football game
pub struct Game {
    pub home_team: Team,
    pub away_team: Team,
    pub score: GameScore,
    pub clock: GameClock,
    pub possession: GamePossession,
    pub log: GameLog,
    coin_flip: CoinFlip
}

impl Game {
    /// Initialize a new game given a home and away team
    pub fn new(home_team: Team, away_team: Team) -> Game {
        // Initialize the Game struct
        Game {
            home_team: home_team,
            away_team: away_team,
            score: GameScore::new(),
            clock: GameClock::new(),
            possession: GamePossession::new(),
            log: GameLog::new(),
            coin_flip: CoinFlip::new()
        }
    }

    /// Log an event to the game log
    pub fn log(&mut self, message: &str) {
        self.log.log(&self.clock, &self.score, &self.possession, message);
    }

    /// Simulate the opening coin flip
    pub fn simulate_opening_coin_fip(&mut self, mut rng: &mut impl Rng) {
        // Simulate the opening coin flip
        self.coin_flip.simulate(&self.away_team, &self.home_team, &mut rng);

        // Set the possession according to the result
        // Update the game log in the process
        self.possession.possession_away = match self.coin_flip.away_team_won() {
            true => {
                self.log(&format!("{} wins the coin flip", self.away_team.abbreviation));
                match self.coin_flip.decision {
                    CoinFlipDecision::KICK => {
                        self.log(&format!("{} will receive the opening kick", self.home_team.abbreviation));
                        false
                    },
                    CoinFlipDecision::RECEIVE => {
                        self.log(&format!("{} will receive the opening kick", self.away_team.abbreviation));
                        true
                    }
                }
            },
            false => {
                self.log(&format!("{} wins the coin flip", self.home_team.abbreviation));
                match self.coin_flip.decision {
                    CoinFlipDecision::KICK => {
                        self.log(&format!("{} will receive the opening kick", self.away_team.abbreviation));
                        true
                    },
                    CoinFlipDecision::RECEIVE => {
                        self.log(&format!("{} will receive the opening kick", self.home_team.abbreviation));
                        false
                    }
                }
            }
        }
    }
}

impl fmt::Display for Game {
    /// Format a `Game` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let team_str = format!("{} {}\n{} {}", self.home_team, self.score.home_team_score, self.away_team, self.score.away_team_score);
        f.write_str(&team_str)
    }
}