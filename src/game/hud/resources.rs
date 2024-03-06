use super::*;

/// Resource for tracking the number of enemies that are supposed to be spawned this round
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct WordPerMinuteTypedIndicator {
    pub wpm: f64,
}

impl Default for WordPerMinuteTypedIndicator {
    fn default() -> WordPerMinuteTypedIndicator {
        WordPerMinuteTypedIndicator { wpm: 0.0 }
    }
}
