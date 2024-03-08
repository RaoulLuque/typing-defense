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
            .register_type::<MainMenuScreenUiElement>()
            .register_type::<SettingsMenuClosed>()
            .register_type::<SettingsMenuOpened>()
            // Add events
            .add_event::<DifficultyChangedEvent>()
            // Add menu States
            .add_state::<MenuState>()
            .add_state::<SettingsMenuState>()
            // Despawn Menu's when other menus are opened or the are exited
            .add_systems(
                OnExit(MenuState::Main),
                despawn_screen::<MainMenuScreenUiElement>,
            )
            .add_systems(
                OnExit(SettingsMenuState::SettingsClosed),
                despawn_screen::<SettingsMenuClosed>,
            )
            .add_systems(
                OnExit(SettingsMenuState::SettingsOpened),
                despawn_screen::<SettingsMenuOpened>,
            )
            // Add systems
            .add_systems(
                OnEnter(SettingsMenuState::SettingsClosed),
                spawn_settings_button,
            )
            .add_systems(
                OnEnter(SettingsMenuState::SettingsOpened),
                spawn_settings_menu,
            )
            .add_systems(
                Update,
                (
                    settings_button_animations,
                    settings_action,
                    change_difficulty,
                ),
            )
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

// State of the settings menu
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum SettingsMenuState {
    #[default]
    SettingsClosed,
    SettingsOpened,
}
