use crate::team::player::Player;

pub struct Defense {
    edge_rushers: Vec<Player>,
    defensive_tackles: Vec<Player>,
    linebackers: Vec<Player>,
    safeties: Vec<Player>,
    cornerbacks: Vec<Player>
}

impl Defense {
    /// Initialize a new empty Defense
    pub fn new() -> Defense {
        Defense {
            edge_rushers: Vec::new(),
            defensive_tackles: Vec::new(),
            linebackers: Vec::new(),
            safeties: Vec::new(),
            cornerbacks: Vec::new()
        }
    }

    /// Get the total number of players of the defensive unit
    pub fn get_num_players(&self) -> usize {
        self.edge_rushers.len() +
        self.defensive_tackles.len() +
        self.linebackers.len() +
        self.safeties.len() +
        self.cornerbacks.len()
    }

    /// Get the overall of the defensive unit
    pub fn get_overall(&self) -> f64 {
        let mut overall: f64 = 0.0_f64;
        for player in self.edge_rushers.iter() {
            overall += player.get_overall();
        }
        for player in self.defensive_tackles.iter() {
            overall += player.get_overall();
        }
        for player in self.linebackers.iter() {
            overall += player.get_overall();
        }
        for player in self.safeties.iter() {
            overall += player.get_overall();
        }
        for player in self.cornerbacks.iter() {
            overall += player.get_overall();
        }
        overall / (self.get_num_players() as f64)
    }
}