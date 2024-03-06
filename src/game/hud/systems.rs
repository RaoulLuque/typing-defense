use super::rounds::resources::{NumberOfEnemiesTypedCurrentRound, RoundStopwatch};
use super::*;

pub fn update_wpm(
    mut wpm: ResMut<WordPerMinuteTypedIndicator>,
    round_stopwatch: Res<RoundStopwatch>,
    number_of_enemies_typed_current_round: Res<NumberOfEnemiesTypedCurrentRound>,
) {
    let elapsed_seconds_this_round = round_stopwatch.stopwatch.elapsed_secs_f64();
    wpm.wpm =
        number_of_enemies_typed_current_round.number as f64 / (elapsed_seconds_this_round / 60.0);
    println!("wps: {:?}", wpm.wpm);
}

pub fn reset_wpm(mut wpm: ResMut<WordPerMinuteTypedIndicator>) {
    wpm.wpm = 0.0;
}
