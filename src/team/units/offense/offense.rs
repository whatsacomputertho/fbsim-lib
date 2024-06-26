use crate::team::player::Player;
use crate::team::players::Players;

pub struct Offense {
    quarterbacks: Vec<Player>,
    runningbacks: Vec<Player>,
    fullbacks: Vec<Player>,
    wide_receivers: Vec<Player>,
    tight_ends: Vec<Player>,
    offensive_line: Vec<Player>
}

impl Offense {
    /// Initialize a new empty offense
    pub fn new() -> Offense {
        Offense {
            quarterbacks: Vec::new(),
            runningbacks: Vec::new(),
            fullbacks: Vec::new(),
            wide_receivers: Vec::new(),
            tight_ends: Vec::new(),
            offensive_line: Vec::new()
        }
    }
}

impl Players for Offense {
    /// Get the total number of players of the defensive unit
    fn get_num_players(&self) -> usize {
        self.quarterbacks.len() +
        self.runningbacks.len() +
        self.fullbacks.len() +
        self.wide_receivers.len() +
        self.tight_ends.len() +
        self.offensive_line.len()
    }

    /// Get the overall of the defensive unit
    fn get_overall(&self) -> f64 {
        let mut sum_overall: f64 = 0.0_f64;
        for player in self.quarterbacks.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.runningbacks.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.fullbacks.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.wide_receivers.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.tight_ends.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.offensive_line.iter() {
            sum_overall += player.get_overall();
        }
        sum_overall / (self.get_num_players() as f64)
    }
}