use crate::team::player::Player;
use crate::team::team::Team;
use crate::game::context::possession::GameDown;
use crate::game::context::context::GameContext;
use crate::game::log::GameLog;
use crate::game::coinflip::{CoinFlip, CoinFlipDecision};
use crate::game::play::Play;

use std::fmt;
use rand::Rng;
use rand::distributions::{Distribution, Bernoulli};
use statrs::distribution::Beta;

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
                        true
                    },
                    CoinFlipDecision::RECEIVE => {
                        self.log(&format!("{} will receive the opening kick", self.away_team.abbreviation));
                        false
                    }
                }
            },
            false => {
                self.log(&format!("{} wins the coin flip", self.home_team.abbreviation));
                match self.coin_flip.decision {
                    CoinFlipDecision::KICK => {
                        self.log(&format!("{} will receive the opening kick", self.away_team.abbreviation));
                        false
                    },
                    CoinFlipDecision::RECEIVE => {
                        self.log(&format!("{} will receive the opening kick", self.home_team.abbreviation));
                        true
                    }
                }
            }
        }
    }

    /// Get the team currently with possession of the ball
    pub fn get_team_in_possession(&self) -> &Team {
        match self.context.possession.possession_away {
            true => &self.away_team,
            false => &self.home_team
        }
    }

    /// Get the team currently defending
    pub fn get_team_defending(&self) -> &Team {
        match self.context.possession.possession_away {
            true => &self.home_team,
            false => &self.away_team
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

    /// Simulate the kick during a kickoff
    fn simulate_kickoff_kick(&mut self, mut rng: &mut impl Rng) -> (bool, usize, String) {
        // Initialize a log message
        let mut message: String = String::new();

        // Get the kicker's name and kicking ability
        let kicker: &Player = self.get_team_in_possession().special_teams.get_kicker();
        let kicker_kicking: usize = kicker.kicking;
        let kicker_name: &str = &kicker.name;

        // Generate the length of the kicker's kickoff
        let dst_kick_length: Beta = Beta::new(kicker_kicking as f64 / 10_f64, 5.0_f64).unwrap();
        let kick_length_seed: f64 = dst_kick_length.sample(&mut rng);
        let kick_length: usize = ((kick_length_seed * 50_f64) + 30_f64) as usize;
        message += &format!("{} kicks {} yards", kicker_name, kick_length);

        // Determine whether a touchback occurred
        if kick_length > 65 {
            message += &format!(" for a touchback");

            // Adjust the game context accordingly
            self.context.possession.flip_possession();
            self.context.possession.set_line_of_possession(25, true);
            self.context.clock.increment_game_clock(5, &mut rng);

            // Log the play and return
            self.log(&message);
            return (true, kick_length, message);
        }

        // Return the kick length and message, continue the play
        return (false, kick_length, message);
    }

    fn simulate_kickoff_catch(&mut self, kick_length: usize, mut rng: &mut impl Rng) -> (bool, String) {
        // Initialize a log message component
        let mut message: String = String::new();

        // Get the kick returner's name
        let returner: &Player = self.get_team_in_possession().special_teams.get_kick_returner();
        let returner_name: &str = &returner.name;
        
        // Determine whether a fair catch occurred
        let mut is_fair_catch: bool = false;
        if kick_length > 55 {
            let dst_fair_catch: Bernoulli = Bernoulli::new(0.5).unwrap();
            is_fair_catch = dst_fair_catch.sample(&mut rng);
            message += &format!(" {} calls for a fair catch", returner_name)
        }
        return (is_fair_catch, message);
    }

    fn simulate_kickoff_return(&mut self, mut rng: &mut impl Rng) -> (bool, isize, usize, String) {
        // Initialize a log message component
        let mut message: String = String::new();

        // Get the kick returner's name, catching, and running ability
        let returner: &Player = self.get_team_in_possession().special_teams.get_kick_returner();
        let returner_running: usize = returner.running;
        let returner_name: &str = &returner.name;

        // Add initial log message value
        message += &format!(", {} fields the kick at the {}", returner_name, self.context.possession.get_yard_line());

        // Get the blocker and defender overall
        let blocker_blocking: usize = self.get_team_in_possession().special_teams.get_kickoff_blockers_blocking();
        let defender_blocking: usize = self.get_team_defending().special_teams.get_kickoff_defenders_blocking();

        // Initialize a distribution for whether the blockers are penetrated
        let blocking_diff: isize = blocker_blocking as isize - defender_blocking as isize;
        let block_penetrated_seed: f64 = (blocking_diff + 9_isize) as f64 / 18_f64;
        let dst_block_penetrated: Bernoulli = Bernoulli::new(block_penetrated_seed).unwrap();

        // Loop until the player is tackled or scores
        let mut player_is_tackled: bool = false;
        let mut total_yards_gained: isize = 0_isize;
        let mut iterations: usize = 0_usize;
        while !player_is_tackled {
            // Determine whether the block was penetrated
            let block_penetrated: bool = dst_block_penetrated.sample(&mut rng);

            // If so, generate the yards closed in by the defender
            if block_penetrated {
                // Get a random defender who penetrated the block
                let blocker: &Player = self.get_team_defending().special_teams.get_random_kickoff_defender(&mut rng);
                let blocker_running: usize = blocker.running;
                let blocker_tackling: usize = blocker.tackling;
                let blocker_name: &str = &blocker.name;

                // Get the yards gained or lost while the defender penetrated the block
                let running_diff: isize = returner_running as isize - blocker_running as isize;
                let yards_gained_seed: f64 = (running_diff + 9_isize) as f64 / 18_f64;
                let dst_yards_gained: Beta = Beta::new(yards_gained_seed, 5.0_f64).unwrap();
                let yards_gained: isize = ((dst_yards_gained.sample(&mut rng) * 10_f64) - 2_f64) as isize;
                total_yards_gained += yards_gained;

                // Determine whether the tackle was made or not
                let running_tackling_diff: isize = returner_running as isize - blocker_tackling as isize;
                let tackle_made_seed: f64 = (running_tackling_diff + 9_isize) as f64 / 18_f64;
                let dst_tackle_made: Bernoulli = Bernoulli::new(tackle_made_seed).unwrap();
                let tackle_made: bool = dst_tackle_made.sample(&mut rng);
                if tackle_made {
                    player_is_tackled = true;
                    message += &format!(", is brought down by {}", blocker_name);
                } else {
                    message += &format!(", breaks {}'s tackle", blocker_name);
                }
            } else {
                // Get the yards gained while the returner ran freely
                let dst_yards_gained: Beta = Beta::new(returner_running as f64 / 10_f64, 5.0_f64).unwrap();
                let yards_gained: isize = (dst_yards_gained.sample(&mut rng) * 10_f64) as isize;
                total_yards_gained += yards_gained;
            }
            iterations += 1_usize;
        }
        return (player_is_tackled, total_yards_gained, iterations, message);
    }

    /// Simulate a kickoff
    fn simulate_kickoff(&mut self, mut rng: &mut impl Rng) -> Result<(), GameError> {
        // If the opening kickoff, then jump to the first quarter
        self.context.clock.start_game();

        // Simulate the kick and get whether a touchback occurred, the kick length, and log message
        let (touchback, kick_length, mut message) = self.simulate_kickoff_kick(&mut rng);
        if touchback {
            return Ok(());
        }

        // Simulate the catch, determine whether a fair catch was called for
        let (is_fair_catch, catch_message) = self.simulate_kickoff_catch(kick_length, &mut rng);
        if is_fair_catch {
            message += &catch_message;

            // Adjust the game context accordingly
            self.context.possession.flip_possession();
            self.context.possession.set_line_of_possession(25, true);
            self.context.clock.increment_game_clock(5, &mut rng);

            // Log the play and return
            self.log(&message);
            self.context.possession.down = GameDown::First;
            return Ok(());
        }

        // Flip the possession & set the new line of possession
        self.context.possession.flip_possession();
        let new_lop = {
            if self.context.possession.direction_left {
                (self.context.possession.line_of_possession - kick_length as isize).abs() as usize
            } else {
                (self.context.possession.line_of_possession + kick_length as isize).abs() as usize
            }
        };
        self.context.possession.set_line_of_possession(new_lop, true);

        // Simulate the kickoff return & append to the message
        let (tackle_made, yards_gained, tackles_broken, return_message) = self.simulate_kickoff_return(&mut rng);
        message += &return_message;

        // Increment the game clock by a few seconds for each broken tackle
        self.context.clock.increment_game_clock(2 * (tackles_broken + 1), &mut rng);
        message += &format!(", gain of {} yards", yards_gained);
        if !tackle_made {
            message += &format!(" for a touchdown!");
        }
        
        // Log the play and increment the down
        self.log(&message);
        self.context.possession.increment(yards_gained);
        Ok(())
    }

    /// Simulate the next play
    pub fn simulate_next_play(&mut self, mut rng: &mut impl Rng) -> Result<(), GameError> {
        // Check if the game is over, if so then error
        if self.context.clock.is_game_over() {
            return Err(GameError::GameOverError(String::from("Cannot simulate next play: Game is finished")));
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