use crate::game::context::clock::GameClock;
use crate::game::context::possession::GamePossession;
use crate::game::context::score::GameScore;

/// # GameContext struct
///
/// A `GameContext` contains all necessary situational information about
/// a football game to motivate playcalling
pub struct GameContext {
    pub clock: GameClock,
    pub possession: GamePossession,
    pub score: GameScore
}

impl GameContext {
    /// Initialize a new GameContext
    pub fn new() -> GameContext {
        GameContext {
            clock: GameClock::new(),
            possession: GamePossession::new(),
            score: GameScore::new()
        }
    }
}