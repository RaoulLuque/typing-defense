use super::*;
use crate::game::RoundState;

pub fn print_menu_message() {
    println!("You are in the menu!");
}

pub fn transition_from_menu_to_in_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut round_state_next_state: ResMut<NextState<RoundState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if app_state.get() != &AppState::InGame {
            app_state_next_state.set(AppState::InGame);
            round_state_next_state.set(RoundState::InRound);
            println!("Entered AppState::Game and RoundState::InRound");
        }
    }
}
