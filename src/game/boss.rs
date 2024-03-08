pub mod components;
use components::*;

pub mod systems;
use systems::*;

use super::*;

pub struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<Boss>()
            // Initialize Resources
            // Add systems for when entering round
            .add_systems(
                OnEnter(RoundState::InRound),
                spawn_boss.after(super::rounds_and_indicators::systems::increase_round_counter),
            )
            // Add systems for when exiting rounds
            .add_systems(OnExit(RoundState::InRound), despawn_boss);
    }
}
