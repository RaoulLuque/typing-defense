use super::*;

/// Number by which the number of enemies per round increases
const NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT: u32 = 3;
/// Number by which the number of enemies per round increases
const ENEMY_BASE_SPEED_INCREMENT: f32 = 7.5;

/// Resets the number of enemies spawned and typed current round and increases the maximum number of
/// enemies spawned this round and base speed according to constants defined in this file.
pub fn increase_round_difficulty(
    mut max_number_of_enemies_this_round: ResMut<MaxNumberOfEnemiesCurrentRound>,
    mut number_of_enemies_spawned_this_round: ResMut<NumberOfEnemiesSpawnedCurrentRound>,
    mut enemy_base_speed_this_round: ResMut<EnemyBaseSpeedCurrentRound>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedCurrentRound>,
) {
    number_of_enemies_spawned_this_round.number = 0;
    number_of_enemies_typed_current_round.number = 0;
    max_number_of_enemies_this_round.number += NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT;
    enemy_base_speed_this_round.speed += ENEMY_BASE_SPEED_INCREMENT;
}

/// Increases the round_counter by one at the start of each round.
/// In between rounds, the round counter is the number of the round before.
pub fn increase_round_counter(mut round_counter: ResMut<RoundCounter>) {
    round_counter.counter += 1;
}

/// Checks if the number of enemies spawn is equal to the max number of enemies this round
/// Is run after all the systems in the enemies module
pub fn check_if_round_is_over(
    max_number_of_enemies_this_round: Res<MaxNumberOfEnemiesCurrentRound>,
    number_of_enemies_typed_this_round: Res<NumberOfEnemiesTypedCurrentRound>,
    mut round_state_next_state: ResMut<NextState<RoundState>>,
) {
    if max_number_of_enemies_this_round.number == number_of_enemies_typed_this_round.number {
        round_state_next_state.set(RoundState::InBetweenRounds);
        println!("Round is over: Entered InBetweenRounds State");
    }
}

pub fn proceed_to_next_round_from_in_between_rounds(
    round_state: Res<State<RoundState>>,
    mut round_state_next_state: ResMut<NextState<RoundState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space)
        && round_state.get() == &RoundState::InBetweenRounds
    {
        round_state_next_state.set(RoundState::InRound);
        println!("Round was started: Entered InRound State");
    }
}
