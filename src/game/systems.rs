use super::*;
use crate::AppState;

pub fn toggle_simulation_state(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::ControlLeft) {
        simulation_state_next_state.set(match simulation_state.get() {
            &SimulationState::Paused => SimulationState::Running,
            &SimulationState::Running => SimulationState::Paused,
        });
        println!("Switched SimulationState");
    }
}
