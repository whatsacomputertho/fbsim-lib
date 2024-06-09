use std::fmt;

/// # Player struct
///
/// A `Player` represents a football player
#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub throwing: usize,
    pub catching: usize,
    pub running: usize,
    pub blocking: usize,
    pub tackling: usize,
    pub kicking: usize
}

impl Player {
    /// Initialize a new player given its attributes
    pub fn new(name: &str, throwing: usize, catching: usize, running: usize, blocking: usize, tackling: usize, kicking: usize) -> Player {
        Player {
            name: String::from(name),
            throwing: throwing,
            catching: catching,
            running: running,
            blocking: blocking,
            tackling: tackling,
            kicking: kicking
        }
    }

    /// Calculate the player's overall rating
    pub fn get_overall(&self) -> f64 {
        (
            self.throwing as f64 +
            self.catching as f64 +
            self.running as f64 +
            self.blocking as f64 +
            self.tackling as f64 +
            self.kicking as f64
        ) / 6 as f64
    }
}

impl fmt::Display for Player {
    /// Format a `Player` as a string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let player_str = format!("{} ({:.2} Overall)", self.name, self.get_overall());
        f.write_str(&player_str)
    }
}