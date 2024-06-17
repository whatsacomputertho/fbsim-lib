use crate::game::score::GameScore;
use crate::game::clock::GameClock;
use crate::game::possession::GamePossession;

/// # Play struct
///
/// A `Play` represents a play in a game of football
pub struct Play {
    score: GameScore,
    clock: GameClock,
    possession: GamePossession,
    message: String
}