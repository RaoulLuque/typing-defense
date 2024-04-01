use crate::{game::SimulationState, AppState};

use bevy::prelude::*;

pub mod systems;
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
            // Despawn Menu's when other menus are opened or they are exited
            .add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(OnEnter(MenuState::Main), spawn_main_menu)
            .add_systems(OnEnter(MenuState::HowToPlay), spawn_how_to_play_screen)
            .add_systems(
                OnEnter(MenuState::HowToPlayTransition),
                transition_to_how_to_play,
            )
            .add_systems(
                OnExit(MenuState::Main),
                despawn_screen::<MainMenuScreenUiElement>,
            )
            .add_systems(
                OnExit(MenuState::HowToPlay),
                despawn_screen::<HowToPlayScreenUiElement>,
            )
            .add_systems(
                OnExit(SimulationState::Paused),
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
            // Add update systems
            .add_systems(
                Update,
                (
                    settings_button_animations,
                    settings_action,
                    change_difficulty,
                    check_if_in_game_menu_is_opened,
                ),
            )
            .add_systems(
                Update,
                transition_from_menu_to_in_game.run_if(in_state(AppState::Menu)),
            )
            .add_systems(
                Update,
                (
                    animate_enemies_in_how_to_play,
                    crate::game::enemies::text::systems::update_text_from_enemies_on_button_press,
                )
                    .run_if(in_state(MenuState::HowToPlay)),
            )
            .add_systems(
                Update,
                (menu_action, menu_button_animations)
                    .run_if(in_state(AppState::Menu).or_else(in_state(SimulationState::Paused))),
            );
    }
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    HowToPlay,
    // State to transition to how to play. Workaround for being able to 'respawn' the how to play screen
    // from the screen itself in order to spawn enemies after having typed one
    HowToPlayTransition,
    #[default]
    NotInTheMenu,
}

// State of the settings menu
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SettingsMenuState {
    #[default]
    SettingsClosed,
    SettingsOpened,
}
