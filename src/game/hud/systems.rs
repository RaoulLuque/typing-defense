use super::enemies::systems::ENEMY_TEXT_FONT_SIZE;
use super::rounds_and_indicators::resources::{ScoreIndicator, WordPerMinuteTypedIndicator};
use super::*;

pub const UI_TEXT_FONT_SIZE: f32 = ENEMY_TEXT_FONT_SIZE;
pub const UI_TEXT_COLOR: Color = Color::DARK_GRAY;
pub const UI_NUMBER_TEXT_COLOR: Color = Color::WHITE;

pub fn spawn_wpm_hud_element(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "WPM: ",
                TextStyle {
                    color: UI_TEXT_COLOR,
                    font_size: UI_TEXT_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: UI_TEXT_FONT_SIZE,
                    color: UI_NUMBER_TEXT_COLOR,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Percent(1.0),
            right: Val::Percent(1.5),
            ..default()
        }),
        WpmText,
        InGameHudUiElement,
    ));
}

pub fn update_wpm_hud_element(
    wpm: Res<WordPerMinuteTypedIndicator>,
    mut wpm_hud_text_query: Query<&mut Text, With<WpmText>>,
) {
    for mut text in &mut wpm_hud_text_query {
        let value = wpm.wpm;
        text.sections[1].value = format!("{value:.0}");
    }
}

pub fn spawn_score_hud_element(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Score: ",
                TextStyle {
                    color: UI_TEXT_COLOR,
                    font_size: UI_TEXT_FONT_SIZE,
                    ..default()
                },
            ),
            TextSection::new(
                "0",
                TextStyle {
                    font_size: UI_TEXT_FONT_SIZE,
                    color: UI_NUMBER_TEXT_COLOR,
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            bottom: Val::Percent(1.0),
            left: Val::Percent(1.5),
            ..default()
        }),
        ScoreText,
        InGameHudUiElement,
    ));
}

pub fn update_score_hud_element(
    score: Res<ScoreIndicator>,
    mut score_hud_text_query: Query<&mut Text, With<ScoreText>>,
) {
    for mut text in &mut score_hud_text_query {
        text.sections[1].value = format!("{}", score.score);
    }
}
