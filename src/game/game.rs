use crate::team::team::Team;
use crate::game::context::possession::GameDown;
use crate::game::context::context::GameContext;
use crate::game::log::GameLog;
use crate::game::coinflip::{CoinFlip, CoinFlipDecision};
use crate::game::play::Play;

use std::fmt;
use rand::Rng;

/// # Game struct
///
/// The `Game` struct represents a football game
pub struct Game {
    pub home_team: Team,
    pub away_team: Team,
    pub context: GameContext,
    pub log: GameLog,
    pub coin_flip: CoinFlip,
    pub plays: Vec<Play>
}

impl Game {
    /// Initialize a new game given a home and away team
    pub fn new(home_team: Team, away_team: Team) -> Game {
        // Initialize the Game struct
        Game {
            home_team: home_team,
            away_team: away_team,
            context: GameContext::new(),
            log: GameLog::new(),
            coin_flip: CoinFlip::new(),
            plays: Vec::new()
        }
    }

    /// Log an event to the game log
    pub fn log(&mut self, message: &str) {
        self.log.log(&self.context, message);
    }

    /// Simulate the opening coin flip
    pub fn simulate_opening_coin_fip(&mut self, mut rng: &mut impl Rng) {
        // Simulate the opening coin flip
        self.coin_flip.simulate(&self.away_team, &self.home_team, &mut rng);

        // Set the possession according to the result
        // Update the game log in the process
        self.context.possession.possession_away = match self.coin_flip.away_team_won() {
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

    /// Simulate an offensive play
    fn simulate_play(&mut self, mut rng: &mut impl Rng) -> Result<(), GameError> {
        Ok(())
    }

    /// Simulate a PAT
    fn simulate_pat(&mut self, mut rng: &mut impl Rng) -> Result<(), GameError> {
        Ok(())
    }

    /// Simulate a kickoff
    fn simulate_kickoff(&mut self, mut rng: &mut impl Rng) -> Result<(), GameError> {
        Ok(())
    }

    /// Simulate the next play
    pub fn simulate_next_play(&mut self, mut rng: &mut impl Rng) -> Result<(), GameError> {
        // Check if the game is over, if so then error
        if self.context.clock.is_game_over() {
            return Err(GameError::GameOverError(String::from("Cannot simulate next play: game is finished")));
        }

        // Simulate the next play based on the down
        match self.context.possession.down {
            GameDown::Kickoff => {
                self.simulate_kickoff(&mut rng)
            },
            GameDown::PointAfter => {
                self.simulate_pat(&mut rng)
            },
            _ => {
                self.simulate_play(&mut rng)
            }
        }
    }
}

impl fmt::Display for Game {
    /// Format a `Game` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let team_str = format!("{} {}\n{} {}", self.home_team, self.context.score.home_team_score, self.away_team, self.context.score.away_team_score);
        f.write_str(&team_str)
    }
}

/// # GameError enum
///
/// The `GameError` enum represents the errors that can be generated
/// during a game.
#[derive(Debug)]
pub enum GameError {
    GameOverError(String),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            GameError::GameOverError(err) => format!("Game Over Error: {}", err)
        };
        f.write_str(&description)
    }
}