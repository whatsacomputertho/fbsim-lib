use std::fmt;

/// # GameQuarter enum
///
/// A `GameQuarter` represents a quarter of a football game
pub enum GameQuarter {
    PREGAME,
    FIRST,
    SECOND,
    HALFTIME,
    THIRD,
    FOURTH,
    OVERTIME,
    POSTGAME
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
            quarter: GameQuarter::PREGAME,
            game_clock_seconds: 900_usize,
            play_clock_seconds: 40_usize
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
            GameQuarter::PREGAME => "Pre",
            GameQuarter::FIRST => "1st",
            GameQuarter::SECOND => "2nd",
            GameQuarter::HALFTIME => "Half",
            GameQuarter::THIRD => "3rd",
            GameQuarter::FOURTH => "4th",
            GameQuarter::OVERTIME => "OT",
            GameQuarter::POSTGAME => "End"
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