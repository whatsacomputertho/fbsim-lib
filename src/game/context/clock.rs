use std::fmt;

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

    /// Return true if the game is over
    pub fn is_game_over(&self) -> bool {
        self.quarter == GameQuarter::Postgame
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