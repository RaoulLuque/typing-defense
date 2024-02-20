use crate::AppState;

use bevy::prelude::*;

mod systems;
use systems::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add systems
            .add_systems(
                Update,
                transition_from_menu_to_in_game.run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnEnter(AppState::Menu), print_menu_message);
    }
}
