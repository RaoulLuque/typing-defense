use self::rounds_and_indicators::resources::StreakIndicator;

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
            bottom: Val::Percent(10.0),
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
        let mut value = format!("{value:.0}");
        if value.len() < 3 {
            for _ in value.len()..3 {
                let mut tmp = " ".to_string();
                tmp.push_str(&value);
                value = tmp;
            }
        }
        text.sections[1].value = value;
    }
}

pub fn spawn_streak_hud_element(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "Streak: ",
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
            bottom: Val::Percent(5.5),
            right: Val::Percent(1.5),
            ..default()
        }),
        StreakText,
        InGameHudUiElement,
    ));
}

pub fn update_streak_hud_element(
    streak_indicator: Res<StreakIndicator>,
    mut streak_hud_text_query: Query<&mut Text, With<StreakText>>,
) {
    for mut text in &mut streak_hud_text_query {
        let streak_number = streak_indicator.number;
        let mut streak_number = format!("{streak_number:.0}");
        if streak_number.len() < 3 {
            for _ in streak_number.len()..3 {
                let mut tmp = " ".to_string();
                tmp.push_str(&streak_number);
                streak_number = tmp;
            }
        }
        text.sections[1].value = streak_number;
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
            position_type: PositionType::Absolute,
            bottom: Val::Percent(1.0),
            right: Val::Percent(1.5),
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
        let score = score.score;
        let mut score = format!("{score:.0}");
        if score.len() < 3 {
            for _ in score.len()..3 {
                let mut tmp = " ".to_string();
                tmp.push_str(&score);
                score = tmp;
            }
        }
        text.sections[1].value = score;
    }
}
