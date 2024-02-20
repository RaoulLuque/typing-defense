pub mod components;
use components::*;

mod systems;
use systems::*;

pub mod resources;
use resources::*;

use super::*;

pub struct CastlePlugin;

impl Plugin for CastlePlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<Castle>()
            .register_type::<NumberOfLivesLeft>()
            // Initialize Resources
            .init_resource::<NumberOfLivesLeft>()
            // Add systems for startup into the game
            .add_systems(OnEnter(AppState::InGame), spawn_castle)
            // Add update systems
            .add_systems(
                Update,
                despawn_castle_if_all_lives_are_gone
                    .run_if(in_state(RoundState::InRound))
                    .after(enemies::systems::enemy_collision_with_castle),
            );
    }
}
