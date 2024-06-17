use std::fmt;

/// # GameDown enum
///
/// A `GameDown` represents a down of a football game
#[derive(PartialEq)]
pub enum GameDown {
    FIRST,
    SECOND,
    THIRD,
    FOURTH,
    PAT,
    KICKOFF
}

/// # GamePossession struct
///
/// A `GamePossession` represents the current possession on the
/// field in a football game.
pub struct GamePossession {
    pub down: GameDown,
    pub first_down_line: isize,
    pub line_of_possession: isize,
    pub possession_away: bool,
    pub direction_left: bool
}

impl GamePossession {
    /// Initialize a zeroed GamePossession
    pub fn new() -> GamePossession {
        GamePossession {
            down: GameDown::KICKOFF,
            first_down_line: 0_isize,
            line_of_possession: 15_isize,
            possession_away: false,
            direction_left: false
        }
    }

    /// Format the down as a string
    pub fn format_down(&self) -> String {
        let down_str = match self.down {
            GameDown::FIRST => "1st",
            GameDown::SECOND => "2nd",
            GameDown::THIRD => "3rd",
            GameDown::FOURTH => "4th",
            GameDown::PAT => "PAT",
            GameDown::KICKOFF => "Kick"
        };
        String::from(down_str)
    }

    /// Calculate the yards to the first down
    pub fn get_yards_to_first(&self) -> isize {
        (self.first_down_line - self.line_of_possession).abs()
    }

    /// Format the down & yards to first as a string
    pub fn format_down_and_yards(&self) -> String {
        let mut down_string: String = self.format_down();
        if self.down == GameDown::PAT || self.down == GameDown::KICKOFF {
            return down_string;
        }
        down_string = format!("{} & {}", down_string, self.get_yards_to_first());
        down_string
    }

    /// Calculate the yards to the endzone
    pub fn get_yards_to_endzone(&self) -> isize {
        match self.direction_left {
            true => (-50_isize - self.line_of_possession).abs(),
            false => (50_isize - self.line_of_possession).abs()
        }
    }

    /// Calculate the current yard line
    pub fn get_yard_line(&self) -> usize {
        (50_isize - self.line_of_possession.abs()) as usize
    }

    /// Calculate whether the team is in their own territory
    pub fn in_own_territory(&self) -> bool {
        match self.direction_left {
            true => self.line_of_possession > 0,
            false => self.line_of_possession <= 0
        }
    }

    /// Format the down, yards to first, and territory as a string
    pub fn format_down_yards_territory(&self) -> String {
        // Format the down and yards
        let down_and_yards: String = self.format_down_and_yards();

        // Calculate the yard line and territory
        let yard_line: usize = self.get_yard_line();
        let territory: String = match self.in_own_territory() {
            true => String::from("own"),
            false => String::from("opp")
        };

        // Format the down, yards, and territory & return
        format!("{} at {} {}", down_and_yards, territory, yard_line)
    }
}

impl fmt::Display for GamePossession {
    /// Format a `GamePossession` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.format_down_yards_territory())
    }
}