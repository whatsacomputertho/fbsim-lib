use crate::game::context::context::GameContext;

/// # GameLog struct
///
/// A `GameLog` stores the log of events which occurred
/// throughout the football game.
pub struct GameLog {
    pub log: Vec<String>
}

impl GameLog {
    /// Initialize an empty GameLog
    pub fn new() -> GameLog {
        GameLog {
            log: Vec::new()
        }
    }

    /// Log to the GameLog
    pub fn log(&mut self, context: &GameContext, message: &str) {
        // Construct the game log string
        let log_string: String = format!("({}, {}, {}) {}", context.clock, context.score, context.possession, String::from(message));
        self.log.push(log_string);
    }
}