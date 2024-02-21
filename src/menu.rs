use crate::AppState;

use bevy::prelude::*;

mod systems;
use systems::*;

mod components;
use components::*;
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register types for debug
            .register_type::<MenuButtonAction>()
            .register_type::<OnMainMenuScreen>()
            // Add menu States
            .add_state::<MenuState>()
            // Despawn Main Menu if Main Menu State is exited
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            // Add systems
            .add_systems(
                Update,
                transition_from_menu_to_in_game.run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(OnEnter(MenuState::Main), spawn_main_menu)
            .add_systems(
                Update,
                (menu_action, menu_button_animations).run_if(in_state(AppState::Menu)),
            );
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    HowToPlay,
    #[default]
    NotInTheMenu,
}
