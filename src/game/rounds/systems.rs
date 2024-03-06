use enemies::resources::EnemySpawnTimer;

use super::*;

/// Number by which the number of enemies increases each round
const NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT: u32 = 3;
/// Number by which the number of enemies increases each round
const ENEMY_BASE_SPEED_INCREMENT: f32 = 7.5;
/// Number of secs by which the interval for enemies spawning decreases each round
const ENEMY_SPAWN_INTERVAL_DECREMENT: f32 = 0.1;

/// Initial speed of enemies at start of game
use super::resources::INITIAL_ENEMY_SPEED;
/// Number of enemies in the first round - super::systems::NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT
use super::resources::INITIAL_MAX_NUMBER_OF_ENEMIES;
// Initial interval for spawning enemies
use enemies::resources::INITIAL_ENEMY_SPAWN_INTERVAL;

/// Resets the number of enemies spawned and typed current round and increases the maximum number of
/// enemies spawned this round and base speed according to constants defined in this file.
pub fn increase_round_difficulty(
    mut max_number_of_enemies_this_round: ResMut<MaxNumberOfEnemiesCurrentRound>,
    mut number_of_enemies_spawned_this_round: ResMut<NumberOfEnemiesSpawnedCurrentRound>,
    mut enemy_base_speed_this_round: ResMut<EnemyBaseSpeedCurrentRound>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedCurrentRound>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    round_counter: Res<RoundCounter>,
) {
    enemy_spawn_timer.timer = Timer::from_seconds(
        (enemies::resources::INITIAL_ENEMY_SPAWN_INTERVAL
            - round_counter.counter as f32 * ENEMY_SPAWN_INTERVAL_DECREMENT)
            .max(0.5),
        TimerMode::Repeating,
    );
    number_of_enemies_spawned_this_round.number = 0;
    number_of_enemies_typed_current_round.number = 0;
    max_number_of_enemies_this_round.number = INITIAL_MAX_NUMBER_OF_ENEMIES
        + round_counter.counter * NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT;
    enemy_base_speed_this_round.speed =
        INITIAL_ENEMY_SPEED + round_counter.counter as f32 * ENEMY_BASE_SPEED_INCREMENT;
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

pub fn tick_round_stopwatch(mut round_stopwatch: ResMut<RoundStopwatch>, time: Res<Time>) {
    round_stopwatch.stopwatch.tick(time.delta());
}

pub fn reset_round_stopwatch(mut round_stopwatch: ResMut<RoundStopwatch>) {
    round_stopwatch.stopwatch.reset();
}
