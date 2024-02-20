mod enemies;
use enemies::EnemiesPlugin;

mod castle;
use castle::CastlePlugin;

use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize App States
            .add_state::<SimulationState>()
            .add_state::<RoundState>()
            // Add own plugins
            .add_plugins(EnemiesPlugin)
            .add_plugins(CastlePlugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum SimulationState {
    Paused,
    #[default]
    Running,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum RoundState {
    #[default]
    InBetweenRounds,
    InRound,
}
