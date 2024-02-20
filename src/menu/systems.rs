use super::*;

pub fn print_menu_message() {
    println!("You are in the menu!");
}

pub fn transition_from_menu_to_in_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if app_state.get() != &AppState::InGame {
            app_state_next_state.set(AppState::InGame);
            println!("Entered AppState::Game");
        }
    }
}
