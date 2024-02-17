mod components;
use components::*;
mod resources;
use resources::*;
mod systems;
use systems::*;

use bevy::prelude::*;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentlyBeingTyped>()
            .register_type::<Enemy>()
            .init_resource::<IsSomethingBeingTyped>()
            .add_systems(Startup, spawn_enemy)
            .add_systems(Update, update_text_from_enemies_on_button_press);
    }
}
