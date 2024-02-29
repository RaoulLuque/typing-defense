mod components;
use components::*;

pub mod resources;
use resources::*;

pub mod systems;
use systems::*;

pub mod movement;

use super::*;

use bevy_common_assets::toml::TomlAssetPlugin;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<CurrentlyBeingTyped>()
            .register_type::<Enemy>()
            .register_type::<Speed>()
            .register_type::<SpriteSize>()
            .register_type::<EnemyType>()
            .register_type::<EnemiesBeingTyped>()
            .register_type::<EnemySpawnTimer>()
            .register_type::<WordsHandle>()
            .register_type::<LastEnemySpawnPoint>()
            .register_type::<movement::components::EnemySpawnPoint>()
            .register_type::<movement::components::PathCheckpointNumber>()
            // Initialize Resources
            .init_resource::<EnemiesBeingTyped>()
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<LastEnemySpawnPoint>()
            // Setup list of words as asset
            .add_plugins(TomlAssetPlugin::<Words>::new(&["words.toml"]))
            .add_systems(Startup, setup_list_of_words_asset)
            // Add update systems that only run if currently in_game and simulation is running
            .add_systems(
                Update,
                (
                    randomly_spawn_enemies_over_time,
                    update_text_from_enemies_on_button_press,
                    tick_enemy_spawn_timer,
                    movement::systems::update_position_of_enemies,
                    animate_enemies,
                    movement::systems::enemy_collision_with_castle,
                    movement::systems::despawn_enemy_if_out_of_screen,
                )
                    .run_if(in_state(AppState::InGame))
                    .run_if(in_state(SimulationState::Running))
                    .before(rounds::systems::check_if_round_is_over),
            );
    }
}
