use super::*;

/// Number of lives the player starts with
const NUMBER_OF_LIVES_AT_START: u8 = 5;

/// Resource used to track the number of lives the player has left
#[derive(Reflect, Resource)]
#[reflect(Resource)]
pub struct NumberOfLivesLeft {
    pub number: u8,
}

impl Default for NumberOfLivesLeft {
    fn default() -> NumberOfLivesLeft {
        NumberOfLivesLeft {
            number: NUMBER_OF_LIVES_AT_START,
        }
    }
}
