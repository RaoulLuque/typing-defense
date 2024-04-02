use enemies::resources::EnemySpawnTimer;
use enemies::text::systems::EnemyTypedEvent;

use crate::menu::systems::Restart;
use crate::menu::GameStartedState;

use super::*;

// Increments and decrements of game values each round for the different difficulties:

// Easy
const NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT_EASY_DIFFICULTY: u32 = 2;
const ENEMY_BASE_SPEED_INCREMENT_EASY_DIFFICULTY: f32 = 3.75;
const ENEMY_SPAWN_INTERVAL_DECREMENT_EASY_DIFFICULTY: f32 = 0.05;

// Medium
const NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT_MEDIUM_DIFFICULTY: u32 = 4;
const ENEMY_BASE_SPEED_INCREMENT_MEDIUM_DIFFICULTY: f32 = 7.5;
const ENEMY_SPAWN_INTERVAL_DECREMENT_MEDIUM_DIFFICULTY: f32 = 0.1;

const NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT_HARD_DIFFICULTY: u32 = 6;
const ENEMY_BASE_SPEED_INCREMENT_HARD_DIFFICULTY: f32 = 10.5;
const ENEMY_SPAWN_INTERVAL_DECREMENT_HARD_DIFFICULTY: f32 = 0.15;

// Initial interval for spawning enemies
use enemies::resources::INITIAL_ENEMY_SPAWN_INTERVAL;

use super::boss::systems::BOSS_WORD_COUNT_MULTIPLIER;

/// Resets the number of enemies spawned, unlived and typed current round
pub fn reset_indicators(
    mut number_of_enemies_spawned_this_round: ResMut<NumberOfEnemiesSpawnedThisRound>,
    mut number_of_enemies_unlived_current_round: ResMut<NumberOfEnemiesUnlivedThisRound>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedThisRound>,
    round_number: Res<RoundNumber>,
) {
    number_of_enemies_spawned_this_round.number = if round_number.number % 10 != 0 {
        0
    } else {
        BOSS_WORD_COUNT_MULTIPLIER * round_number.number
    };
    number_of_enemies_unlived_current_round.number = 0;
    number_of_enemies_typed_current_round.number = 0;
}

/// Increases the maximum number of enemies spawned this round and base speed according to constants defined in this file.
pub fn increase_round_difficulty(
    mut max_number_of_enemies_this_round: ResMut<MaxNumberOfEnemiesCurrentRound>,
    mut enemy_base_speed_this_round: ResMut<EnemyBaseSpeedCurrentRound>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    round_number: Res<RoundNumber>,
    difficulty_indicator: Res<DifficultyIndicator>,
) {
    enemy_spawn_timer.timer = Timer::from_seconds(
        (INITIAL_ENEMY_SPAWN_INTERVAL
            - round_number.number as f32
                * match difficulty_indicator.difficulty {
                    Difficulty::Easy => ENEMY_SPAWN_INTERVAL_DECREMENT_EASY_DIFFICULTY,
                    Difficulty::Medium => ENEMY_SPAWN_INTERVAL_DECREMENT_MEDIUM_DIFFICULTY,
                    Difficulty::Hard => ENEMY_SPAWN_INTERVAL_DECREMENT_HARD_DIFFICULTY,
                })
        .max(0.5),
        TimerMode::Repeating,
    );
    // Every 10 rounds is boss round and no enemies should be spawned
    max_number_of_enemies_this_round.number = if round_number.number % 10 != 0 {
        INITIAL_MAX_NUMBER_OF_ENEMIES
            + round_number.number
                * match difficulty_indicator.difficulty {
                    Difficulty::Easy => NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT_EASY_DIFFICULTY,
                    Difficulty::Medium => NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT_MEDIUM_DIFFICULTY,
                    Difficulty::Hard => NUMBER_OF_ENEMIES_PER_ROUND_INCREMENT_HARD_DIFFICULTY,
                }
    } else {
        BOSS_WORD_COUNT_MULTIPLIER * round_number.number
    };
    enemy_base_speed_this_round.speed = if round_number.number % 10 != 0 {
        INITIAL_ENEMY_SPEED
            + round_number.number as f32
                * match difficulty_indicator.difficulty {
                    Difficulty::Easy => ENEMY_BASE_SPEED_INCREMENT_EASY_DIFFICULTY,
                    Difficulty::Medium => ENEMY_BASE_SPEED_INCREMENT_MEDIUM_DIFFICULTY,
                    Difficulty::Hard => ENEMY_BASE_SPEED_INCREMENT_HARD_DIFFICULTY,
                }
    } else {
        INITIAL_ENEMY_SPEED * 0.5
    };
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
pub fn update_score_and_number_of_enemies_typed(
    mut score: ResMut<ScoreIndicator>,
    mut enemy_typed_event: EventReader<EnemyTypedEvent>,
    mut number_of_enemies_unlived_current_round: ResMut<NumberOfEnemiesUnlivedThisRound>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedThisRound>,
    wpm: Res<WordPerMinuteTypedIndicator>,
    streak_counter: Res<StreakIndicator>,
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
        number_of_enemies_unlived_current_round.number += 1;
        number_of_enemies_typed_current_round.number += 1;
    }
}

pub fn set_states_on_restart(
    mut restart_event_reader: EventReader<Restart>,
    mut round_state_next_state: ResMut<NextState<RoundState>>,
    mut game_started_state_next_state: ResMut<NextState<GameStartedState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    for _ in restart_event_reader.read() {
        game_started_state_next_state.set(GameStartedState::GameHasNotStarted);
        round_state_next_state.set(RoundState::InBetweenRounds);
        app_state_next_state.set(AppState::Menu);
        simulation_state_next_state.set(SimulationState::Running);
    }
}

pub fn reset_score_and_indicators_on_restart(
    mut restart_event_reader: EventReader<Restart>,
    mut number_of_enemies_unlived_current_round: ResMut<NumberOfEnemiesUnlivedThisRound>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedThisRound>,
    mut round_number: ResMut<RoundNumber>,
    mut score: ResMut<ScoreIndicator>,
    mut streak: ResMut<StreakIndicator>,
) {
    for _ in restart_event_reader.read() {
        round_number.number = 0;
        score.score = 0;
        streak.number = 0;
        number_of_enemies_unlived_current_round.number = 0;
        number_of_enemies_typed_current_round.number = 0;
    }
}
