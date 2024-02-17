use bevy::prelude::*;

#[derive(Resource)]
pub struct IsSomethingBeingTyped {
    pub indicator: bool,
}

impl Default for IsSomethingBeingTyped {
    fn default() -> IsSomethingBeingTyped {
        IsSomethingBeingTyped { indicator: false }
    }
}
