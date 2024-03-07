use enemies::resources::EnemySpawnTimer;
use enemies::text::systems::EnemyTypedEvent;

use super::*;

/// Number by which the number of enemies increases each round
const NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT: u32 = 3;
/// Number by which the number of enemies increases each round
const ENEMY_BASE_SPEED_INCREMENT: f32 = 7.5;
/// Number of secs by which the interval for enemies spawning decreases each round
const ENEMY_SPAWN_INTERVAL_DECREMENT: f32 = 0.1;

// Initial interval for spawning enemies
use enemies::resources::INITIAL_ENEMY_SPAWN_INTERVAL;

/// Resets the number of enemies spawned, unlived and typed current round
pub fn reset_indicators(
    mut number_of_enemies_spawned_this_round: ResMut<NumberOfEnemiesSpawnedThisRound>,
    mut number_of_enemies_unlived_current_round: ResMut<NumberOfEnemiesUnlivedThisRound>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedThisRound>,
) {
    number_of_enemies_spawned_this_round.number = 0;
    number_of_enemies_unlived_current_round.number = 0;
    number_of_enemies_typed_current_round.number = 0;
}

/// Increases the maximum number of enemies spawned this round and base speed according to constants defined in this file.
pub fn increase_round_difficulty(
    mut max_number_of_enemies_this_round: ResMut<MaxNumberOfEnemiesCurrentRound>,

    mut enemy_base_speed_this_round: ResMut<EnemyBaseSpeedCurrentRound>,

    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    round_counter: Res<RoundNumber>,
) {
    enemy_spawn_timer.timer = Timer::from_seconds(
        (INITIAL_ENEMY_SPAWN_INTERVAL
            - round_counter.number as f32 * ENEMY_SPAWN_INTERVAL_DECREMENT)
            .max(0.5),
        TimerMode::Repeating,
    );
    max_number_of_enemies_this_round.number = INITIAL_MAX_NUMBER_OF_ENEMIES
        + round_counter.number * NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT;
    enemy_base_speed_this_round.speed =
        INITIAL_ENEMY_SPEED + round_counter.number as f32 * ENEMY_BASE_SPEED_INCREMENT;
}

/// Increases the round_counter by one at the start of each round.
/// In between rounds, the round counter is the number of the round before.
pub fn increase_round_counter(mut round_counter: ResMut<RoundNumber>) {
    round_counter.number += 1;
}

/// Checks if the number of enemies spawn is equal to the max number of enemies this round
/// Is run after all the systems in the enemies module
pub fn check_if_round_is_over(
    max_number_of_enemies_this_round: Res<MaxNumberOfEnemiesCurrentRound>,
    number_of_enemies_typed_this_round: Res<NumberOfEnemiesUnlivedThisRound>,
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

pub fn update_wpm(
    mut wpm: ResMut<WordPerMinuteTypedIndicator>,
    round_stopwatch: Res<RoundStopwatch>,
    number_of_enemies_typed_current_round: Res<NumberOfEnemiesTypedThisRound>,
) {
    let elapsed_seconds_this_round = round_stopwatch.stopwatch.elapsed_secs_f64();
    wpm.wpm =
        number_of_enemies_typed_current_round.number as f64 / (elapsed_seconds_this_round / 60.0);
}

pub fn reset_wpm(mut wpm: ResMut<WordPerMinuteTypedIndicator>) {
    wpm.wpm = 0.0;
}

/// When an enemy is typed, the score is increased by:
///
/// current wpm * (streak counter / 50 + 1) * (round number / 10 + 1) * difficulty multiplier
///
/// Where the difficulty multiplier is 1 for easy, 2 for medium and 3 for hard and operations are
/// done as f64 and converted to u64 at the end.
pub fn update_score(
    mut score: ResMut<ScoreIndicator>,
    mut enemy_typed_event: EventReader<EnemyTypedEvent>,
    wpm: Res<WordPerMinuteTypedIndicator>,
    streak_counter: Res<StreakNumberThisRound>,
    round_number: Res<RoundNumber>,
    difficulty: Res<DifficultyIndicator>,
) {
    for _ in enemy_typed_event.read() {
        score.score += (match difficulty.difficulty {
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
        } as f64
            * wpm.wpm
            * (streak_counter.number as f64 / 50.0 + 1.0)
            * (round_number.number as f64 / 10.0 + 1.0)) as u64;
    }
}
