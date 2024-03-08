mod enemies;
use enemies::EnemiesPlugin;

mod castle;
use castle::CastlePlugin;

mod rounds_and_indicators;
use rounds_and_indicators::RoundsAndIndicatorsPlugin;

mod hud;
use hud::HUDPlugin;

mod decorations;
use decorations::DecorationsPlugin;

mod systems;
use systems::*;

mod effects;
use effects::EffectsPlugin;

mod boss;
use boss::BossPlugin;

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
            .add_plugins(RoundsAndIndicatorsPlugin)
            .add_plugins(EffectsPlugin)
            .add_plugins(DecorationsPlugin)
            .add_plugins(HUDPlugin)
            .add_plugins(BossPlugin)
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

/// Starts in Running.
///
/// Switches when ctrl is pressed. In Paused, enemies don't move and are not typeable.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum SimulationState {
    Paused,
    #[default]
    Running,
}

/// Starts InBetweenRounds and enters InRound when Start Game is pressed.
///
/// Alternates between the two states when all enemies are typed and next round is started with space.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum RoundState {
    #[default]
    InBetweenRounds,
    InRound,
}
