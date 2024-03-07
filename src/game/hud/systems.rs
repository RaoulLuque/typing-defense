use super::enemies::systems::ENEMY_TEXT_FONT_SIZE;
use super::rounds::resources::{NumberOfEnemiesTypedCurrentRound, RoundStopwatch};
use super::*;

pub const UI_TEXT_FONT_SIZE: f32 = ENEMY_TEXT_FONT_SIZE;

pub fn update_wpm(
    mut wpm: ResMut<WordPerMinuteTypedIndicator>,
    round_stopwatch: Res<RoundStopwatch>,
    number_of_enemies_typed_current_round: Res<NumberOfEnemiesTypedCurrentRound>,
) {
    let elapsed_seconds_this_round = round_stopwatch.stopwatch.elapsed_secs_f64();
    wpm.wpm =
        number_of_enemies_typed_current_round.number as f64 / (elapsed_seconds_this_round / 60.0);
}

pub fn reset_wpm(mut wpm: ResMut<WordPerMinuteTypedIndicator>) {
    wpm.wpm = 0.0;
}

pub fn spawn_wpm_hud_element(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "WPM: ",
                TextStyle {
                    font_size: UI_TEXT_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: UI_TEXT_FONT_SIZE,
                    color: Color::DARK_GRAY,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(0.9),
            right: Val::Percent(0.9),
            ..default()
        }),
        WpmText,
        InRoundHudUiElement,
    ));
}

pub fn update_wpm_hud_element(
    wpm: Res<WordPerMinuteTypedIndicator>,
    mut query: Query<&mut Text, With<WpmText>>,
) {
    for mut text in &mut query {
        let value = wpm.wpm;
        text.sections[1].value = format!("{value:.0}");
    }
}
