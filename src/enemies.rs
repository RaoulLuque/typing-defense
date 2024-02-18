mod components;
use components::*;
mod resources;
use resources::*;
mod systems;
use systems::*;

use bevy::prelude::*;

use bevy_common_assets::toml::TomlAssetPlugin;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<CurrentlyBeingTyped>()
            .register_type::<Enemy>()
            .register_type::<EnemiesBeingTyped>()
            .register_type::<EnemySpawnTimer>()
            .register_type::<NumberOfEnemies>()
            .register_type::<WordsHandle>()
            // Initialize Resources
            .init_resource::<EnemiesBeingTyped>()
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<NumberOfEnemies>()
            // Setup list of words as asset
            .add_plugins(TomlAssetPlugin::<Words>::new(&["words.toml"]))
            .add_systems(Startup, setup_assets.before(spawn_enemy))
            // Add update systems
            .add_systems(Update, spawn_enemy)
            .add_systems(Update, update_text_from_enemies_on_button_press)
            .add_systems(Update, tick_enemy_spawn_timer);
    }
}
