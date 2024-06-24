use std::fmt;
use rand::Rng;
use rand::distributions::Distribution;
use statrs::distribution::Beta;

/// # GameQuarter enum
///
/// A `GameQuarter` represents a quarter of a football game
#[derive(PartialEq)]
pub enum GameQuarter {
    Pregame,
    First,
    Second,
    Halftime,
    Third,
    Fourth,
    Overtime,
    Postgame
}

/// # GameClock struct
///
/// A `GameClock` represents the score of a football game
pub struct GameClock {
    pub quarter: GameQuarter,
    pub game_clock_seconds: usize,
    pub play_clock_seconds: usize
}

impl GameClock {
    /// Initialize a zeroed GameClock
    pub fn new() -> GameClock {
        GameClock {
            quarter: GameQuarter::Pregame,
            game_clock_seconds: 900_usize,
            play_clock_seconds: 40_usize
        }
    }

    /// Jump to the first quarter if in pregame
    pub fn start_game(&mut self) {
        if self.quarter == GameQuarter::Pregame {
            self.quarter = GameQuarter::First;
        }
    }

    /// Return true if the game is over
    pub fn is_game_over(&self) -> bool {
        self.quarter == GameQuarter::Postgame
    }

    /// Increment the game clock randomly given an expected number of seconds
    pub fn increment_game_clock(&mut self, expected_seconds: usize, mut rng: &mut impl Rng) {
        // Initialize a variable tracking the expected seconds, max 50
        let mean_seconds: usize = if expected_seconds > 30 { 30_usize } else { expected_seconds };

        // Map to a beta distribution centered at that mean & generate a play time
        let dst_play_time: Beta = Beta::new(mean_seconds as f64 / 30_usize as f64, 5.0_f64).unwrap();
        let play_time_seconds: usize = (dst_play_time.sample(&mut rng) * 30_f64) as usize;
        println!("Play time: {}", play_time_seconds);

        // Subtract the play time from the game clock
        if play_time_seconds > self.game_clock_seconds {
            self.game_clock_seconds = 0;
        } else {
            self.game_clock_seconds = self.game_clock_seconds - play_time_seconds;
        }
    }

    /// Format the game clock as a string
    pub fn format_game_clock(&self) -> String {
        let minutes = self.game_clock_seconds / 60;
        let seconds = self.game_clock_seconds % 60;
        let seconds_str = if seconds > 9 {
            format!("{}", seconds)
        } else {
            format!("0{}", seconds)
        };
        format!("{}:{}", minutes, seconds_str)
    }

    /// Format the quarter as a string
    pub fn format_quarter(&self) -> String {
        let quarter_str = match self.quarter {
            GameQuarter::Pregame => "Pre",
            GameQuarter::First => "1st",
            GameQuarter::Second => "2nd",
            GameQuarter::Halftime => "Half",
            GameQuarter::Third => "3rd",
            GameQuarter::Fourth => "4th",
            GameQuarter::Overtime => "OT",
            GameQuarter::Postgame => "End"
        };
        String::from(quarter_str)
    }
}

impl fmt::Display for GameClock {
    /// Format a `GameClock` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let clock_str = format!("{} {} [{}]", self.format_quarter(), self.format_game_clock(), self.play_clock_seconds);
        f.write_str(&clock_str)
    }
}