use crate::team::player::Player;
use crate::team::players::Players;

use rand::Rng;
use rand::prelude::SliceRandom;

pub struct SpecialTeams {
    pub kickers: Vec<Player>,
    pub kick_returners: Vec<Player>,
    pub punters: Vec<Player>,
    pub punt_returners: Vec<Player>,
    pub linemen: Vec<Player>,
    pub defenders: Vec<Player>,
    pub extras: Vec<Player>
}

impl SpecialTeams {
    /// Initialize an empty SpecialTeams
    pub fn new() -> SpecialTeams {
        SpecialTeams {
            kickers: Vec::new(),
            kick_returners: Vec::new(),
            punters: Vec::new(),
            punt_returners: Vec::new(),
            linemen: Vec::new(),
            defenders: Vec::new(),
            extras: Vec::new()
        }
    }

    /// Get the kicker
    pub fn get_kicker(&self) -> &Player {
        // Return the first kicker we find in the kickers list
        for kicker in self.kickers.iter() {
            return &kicker;
        }

        // If none then panic
        panic!("Could not find any kicker");
    }

    /// Get the kick returner
    pub fn get_kick_returner(&self) -> &Player {
        // Return the first kick returner we find in the kick returners list
        for kick_returner in self.kick_returners.iter() {
            return &kick_returner;
        }

        // If none then panic
        panic!("Could not find any kick returner");
    }

    /// Get kickoff defenders blocking
    pub fn get_kickoff_defenders_blocking(&self) -> usize {
        let mut total_blocking: usize = 0_usize;
        for player in self.defenders.iter() {
            total_blocking += player.blocking;
        }
        for player in self.extras.iter() {
            total_blocking += player.blocking;
        }
        let num_defenders = self.defenders.get_num_players();
        let num_extras = self.extras.get_num_players();
        (total_blocking as f64 / (num_defenders + num_extras) as f64) as usize
    }

    /// Get kickoff blockers blocking
    pub fn get_kickoff_blockers_blocking(&self) -> usize {
        let mut total_blocking: usize = 0_usize;
        for player in self.linemen.iter() {
            total_blocking += player.blocking;
        }
        for player in self.extras.iter() {
            total_blocking += player.blocking;
        }
        let num_linemen = self.linemen.get_num_players();
        let num_extras = self.extras.get_num_players();
        (total_blocking as f64 / (num_linemen + num_extras) as f64) as usize
    }

    /// Get random kickoff defender
    pub fn get_random_kickoff_defender(&self, mut rng: &mut impl Rng) -> &Player {
        let total_players = self.defenders.get_num_players() + self.extras.get_num_players();
        let rand_num = rng.gen_range(0..total_players);
        if rand_num < self.defenders.len() {
            &self.defenders.get(rand_num).unwrap()
        } else {
            &self.extras.get(rand_num - self.defenders.len()).unwrap()
        }
    }
}

impl Players for SpecialTeams {
    /// Get the total number of players of the special teams unit
    fn get_num_players(&self) -> usize {
        self.kickers.len() +
        self.kick_returners.len() +
        self.punters.len() +
        self.punt_returners.len() +
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
        for player in self.kick_returners.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.punters.iter() {
            sum_overall += player.get_overall();
        }
        for player in self.punt_returners.iter() {
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