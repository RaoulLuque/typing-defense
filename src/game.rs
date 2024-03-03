mod enemies;
use enemies::EnemiesPlugin;

mod castle;
use castle::CastlePlugin;

mod rounds;
use rounds::RoundsPlugin;

mod hud;
use hud::HUDPlugin;

mod decorations;
use decorations::DecorationsPlugin;

mod systems;
use systems::*;

mod effects;
use effects::EffectsPlugin;

use bevy::prelude::*;

use crate::AppState;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum InputHandlingSystemSet {
    BeforeInputHandling,
    InputHandling,
    AfterInputHandling,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialize App States
            .add_state::<SimulationState>()
            .add_state::<RoundState>()
            // Add own plugins
            .add_plugins(EnemiesPlugin)
            .add_plugins(CastlePlugin)
            .add_plugins(RoundsPlugin)
            .add_plugins(EffectsPlugin)
            .add_plugins(DecorationsPlugin)
            .add_plugins(HUDPlugin)
            // Configure System Sets
            .configure_sets(
                Update,
                // chain() will ensure sets run in the order they are listed
                (
                    InputHandlingSystemSet::BeforeInputHandling,
                    InputHandlingSystemSet::InputHandling,
                    InputHandlingSystemSet::AfterInputHandling,
                )
                    .chain()
                    .run_if(in_state(RoundState::InRound))
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Add system for changing simulation states - only possible if:
            // 1. InGame
            // 2. InRound (in order not to confuse when in between rounds since pausing seems uneccessary)
            .add_systems(
                Update,
                toggle_simulation_state
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(RoundState::InRound)),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum SimulationState {
    Paused,
    #[default]
    Running,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum RoundState {
    InBetweenRounds,
    #[default]
    InRound,
}
