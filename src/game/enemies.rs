pub mod components;
use components::*;

pub mod resources;
use resources::*;

pub mod systems;
use systems::*;

pub mod movement;
pub mod text;

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
            .register_type::<text::components::CollidingWith>()
            // Add events
            .add_event::<text::systems::EnemyTypedEvent>()
            // Initialize Resources
            .init_resource::<EnemiesBeingTyped>()
            .init_resource::<EnemySpawnTimer>()
            .init_resource::<LastEnemySpawnPoint>()
            // Setup list of words as asset
            .add_plugins(TomlAssetPlugin::<Words>::new(&["words.toml"]))
            .add_systems(Startup, text::systems::setup_list_of_words_asset)
            // Add update systems that only run if currently in_game and simulation is running
            .add_systems(
                Update,
                (
                    // Reset colliding text if necessary
                    text::systems::lower_text_stepwise_when_colliding_enemy_is_removed,
                    text::systems::reset_text_height_when_enemies_passed_each_other,
                    text::systems::check_if_colliding_text_has_moved,
                )
                    .in_set(super::InputHandlingSystemSet::AfterInputHandling),
            )
            .add_systems(
                Update,
                (
                    randomly_spawn_enemies_over_time,
                    text::systems::update_text_from_enemies_on_button_press,
                    text::systems::handle_text_when_enemies_collide,
                    tick_enemy_spawn_timer,
                    movement::systems::update_position_of_enemies_and_bosses,
                    animate_enemies,
                    movement::systems::enemy_collision_with_castle,
                    movement::systems::despawn_enemy_if_out_of_screen,
                )
                    .in_set(super::InputHandlingSystemSet::InputHandling),
            );
    }
}
