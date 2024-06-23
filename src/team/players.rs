use crate::team::player::Player;

/// # Players trait
///
/// A `Players` implementation is representative of a collection of players.
pub trait Players {
    /// Expected to generate the overall rating of the collection of players
    fn get_overall(&self) -> f64;

    /// Expected to determine the total number of players in the collection
    fn get_num_players(&self) -> usize;
}

impl Players for Vec<Player> {
    /// Generates the overall rating of the collection of players
    fn get_overall(&self) -> f64 {
        // Initialize a sum of overall ratings
        let mut sum_overall: f64 = 0.0_f64;

        // Loop through the vec of players and add their overall to the sum
        for player in self.iter() {
            sum_overall += player.get_overall();
        }

        // Return the sum divided by the number of players
        sum_overall / (self.get_num_players() as f64)
    }

    /// Determines the total number of players in the collection
    fn get_num_players(&self) -> usize {
        // Return the len of the vec
        self.len()
    }
}