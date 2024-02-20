mod components;
use components::*;
mod resources;
use resources::*;
mod systems;
use systems::*;

use crate::AppState;

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
            .register_type::<EnemiesBeingTyped>()
            .register_type::<EnemySpawnTimer>()
            .register_type::<NumberOfEnemies>()
            .register_type::<WordsHandle>()
            .register_type::<LastEnemySpawnPoint>()
            .register_type::<EnemySpawnPoint>()
            // Initialize Resources
            .init_resource::<EnemiesBeingTyped>()
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<NumberOfEnemies>()
            .init_resource::<LastEnemySpawnPoint>()
            // Setup list of words as asset
            .add_plugins(TomlAssetPlugin::<Words>::new(&["words.toml"]))
            .add_systems(Startup, setup_assets)
            // Add update systems
            .add_systems(
                Update,
                (
                    randomly_spawn_enemies_over_time,
                    update_text_from_enemies_on_button_press,
                    tick_enemy_spawn_timer,
                    update_position_of_enemies,
                    enemy_collision_with_castle,
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}
