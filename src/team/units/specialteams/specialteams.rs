use crate::team::player::Player;
use crate::team::players::Players;

pub struct SpecialTeams {
    kickers: Vec<Player>,
    punters: Vec<Player>,
    linemen: Vec<Player>,
    defenders: Vec<Player>,
    extras: Vec<Player>
}

impl SpecialTeams {
    /// Initialize an empty SpecialTeams
    pub fn new() -> SpecialTeams {
        SpecialTeams {
            kickers: Vec::new(),
            punters: Vec::new(),
            linemen: Vec::new(),
            defenders: Vec::new(),
            extras: Vec::new()
        }
    }
}

impl Players for SpecialTeams {
    /// Get the total number of players of the special teams unit
    fn get_num_players(&self) -> usize {
        self.kickers.len() +
        self.punters.len() +
        self.linemen.len() +
        self.defenders.len() +
        self.extras.len()
    }

    /// Get the overall of the special teams unit
    fn get_overall(&self) -> f64 {
        let mut sum_overall: f64 = 0.0_f64;
        for player in self.kickers.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.punters.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.linemen.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.defenders.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.extras.iter() {
            sum_overall += player.get_overall();
        }
        sum_overall / (self.get_num_players() as f64)
    }
}