use std::fmt;

/// # GameDown enum
///
/// A `GameDown` represents a down of a football game
#[derive(PartialEq)]
pub enum GameDown {
    First,
    Second,
    Third,
    Fourth,
    PointAfter,
    Kickoff
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
            down: GameDown::Kickoff,
            first_down_line: 0_isize,
            line_of_possession: 15_isize,
            possession_away: false,
            direction_left: false
        }
    }

    /// Flip the possession
    pub fn flip_possession(&mut self) {
        self.possession_away = !self.possession_away;
        self.direction_left = !self.direction_left;
    }

    /// Set the line of possession
    pub fn set_line_of_possession(&mut self, yard_line: usize, own: bool) {
        // Initialize a local mutable variable for the line of possession
        let mut line_of_pos: isize = yard_line as isize;

        // Ensure the yard line is valid
        if line_of_pos > 50_isize {
            line_of_pos = 50_isize;
        }

        // Negate the line of possession based on direction & side of field
        if own {
            if !self.direction_left {
                line_of_pos = line_of_pos * -1_isize;
            }
        } else {
            if self.direction_left {
                line_of_pos = line_of_pos * -1_isize;
            }
        }
        self.line_of_possession = line_of_pos;
    }

    /// Reset the first down line based on the line of possession
    pub fn reset_first_down_line(&mut self) {
        // If the line of possession is +/- 10 from a goal line
        // then set it to the goal line.  Otherwise set it to the line
        // of possetion +/- 10 depending on direction.
        if self.direction_left && self.line_of_possession < -40_isize {
            self.first_down_line = -50_isize;
        } else if !self.direction_left && self.line_of_possession > 40_isize {
            self.first_down_line = 50_isize;
        } else {
            match self.direction_left {
                true => { self.first_down_line = self.line_of_possession - 10_isize; },
                false => { self.first_down_line = self.line_of_possession + 10_isize; },
            };
        }
    }

    /// Increment the down after a play completes
    pub fn increment(&mut self, yards: isize) {
        // Add the yards to the line of possession
        if self.direction_left {
            self.line_of_possession -= yards;
        } else {
            self.line_of_possession += yards;
        }

        // Max out the line of possession at += 50
        if self.line_of_possession > 50_isize {
            self.line_of_possession = 50_isize;
        } else if self.line_of_possession < -50_isize {
            self.line_of_possession = -50_isize;
        }

        // Check if a touchdown was achieved
        if self.direction_left && self.line_of_possession == -50 {
            self.down = GameDown::PointAfter;
            return;
        } else if !self.direction_left && self.line_of_possession == 50 {
            self.down = GameDown::PointAfter;
            return;
        }

        // Check if a first down was achieved
        if self.direction_left && self.line_of_possession <= self.first_down_line {
            self.down = GameDown::First;
            self.reset_first_down_line();
            return;
        } else if !self.direction_left && self.line_of_possession >= self.first_down_line {
            self.down = GameDown::First;
            self.reset_first_down_line();
            return;
        }

        // Check if this was fourth down
        if self.down == GameDown::Fourth {
            self.possession_away = !self.possession_away;
            self.down = GameDown::First;
            self.reset_first_down_line();
            return;
        }

        // Increment the down
        self.down = match self.down {
            GameDown::PointAfter => GameDown::Kickoff,
            GameDown::Kickoff => GameDown::First,
            GameDown::First => GameDown::Second,
            GameDown::Second => GameDown::Third,
            GameDown::Third => GameDown::Fourth,
            GameDown::Fourth => GameDown::First
        }
    }

    /// Format the down as a string
    pub fn format_down(&self) -> String {
        let down_str = match self.down {
            GameDown::First => "1st",
            GameDown::Second => "2nd",
            GameDown::Third => "3rd",
            GameDown::Fourth => "4th",
            GameDown::PointAfter => "PAT",
            GameDown::Kickoff => "Kick"
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
        if self.down == GameDown::PointAfter || self.down == GameDown::Kickoff {
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