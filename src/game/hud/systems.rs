use self::rounds_and_indicators::resources::{RoundNumber, StreakIndicator};

use super::enemies::systems::ENEMY_TEXT_FONT_SIZE;
use super::rounds_and_indicators::resources::{ScoreIndicator, WordPerMinuteTypedIndicator};
use super::*;

pub const UI_TEXT_FONT_SIZE: f32 = ENEMY_TEXT_FONT_SIZE * 0.50;
pub const UI_TEXT_COLOR: Color = Color::DARK_GRAY;
pub const UI_NUMBER_TEXT_COLOR: Color = Color::WHITE;

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

pub fn update_round_number_hud_element(
    round_number: Res<RoundNumber>,
    mut round_number_hud_text_query: Query<&mut Text, With<RoundNumberText>>,
) {
    for mut text in &mut round_number_hud_text_query {
        let round_number = round_number.number;
        let mut round_number = format!("{round_number:.0}");
        if round_number.len() < 3 {
            for _ in round_number.len()..3 {
                let mut tmp = " ".to_string();
                tmp.push_str(&round_number);
                round_number = tmp;
            }
        }
        text.sections[1].value = round_number;
    }
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

pub fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                z_index: ZIndex::Global(0),
                ..default()
            },
            InGameHudParent,
            Name::new("Hud Banner parent"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(48.0),
                            height: Val::Percent(10.0),
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            margin: UiRect::top(Val::Percent(0.5)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("ui/hud/hud_banner.png")),
                ))
                .with_children(|parent| {
                    parent.spawn((
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
                            width: Val::Percent(20.0),
                            margin: UiRect::new(
                                Val::Percent(5.0),
                                Val::Percent(0.0),
                                Val::Percent(0.0),
                                Val::Percent(3.8),
                            ),
                            ..default()
                        }),
                        ScoreText,
                        InGameHudUiElement,
                    ));
                    parent.spawn((
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
                                "  0",
                                TextStyle {
                                    font_size: UI_TEXT_FONT_SIZE,
                                    color: UI_NUMBER_TEXT_COLOR,
                                    ..default()
                                },
                            ),
                        ])
                        .with_style(Style {
                            width: Val::Percent(20.0),
                            margin: UiRect::bottom(Val::Percent(3.8)),
                            ..default()
                        }),
                        StreakText,
                        InGameHudUiElement,
                    ));
                    parent.spawn((
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
                            width: Val::Percent(20.0),
                            margin: UiRect::new(
                                Val::Percent(0.0),
                                Val::Percent(0.0),
                                Val::Percent(0.0),
                                Val::Percent(3.8),
                            ),
                            ..default()
                        }),
                        WpmText,
                        InGameHudUiElement,
                    ));

                    parent.spawn((
                        // Create a TextBundle that has a Text with a list of sections.
                        TextBundle::from_sections([
                            TextSection::new(
                                "Round: ",
                                TextStyle {
                                    color: UI_TEXT_COLOR,
                                    font_size: UI_TEXT_FONT_SIZE,
                                    ..default()
                                },
                            ),
                            TextSection::new(
                                "  0",
                                TextStyle {
                                    font_size: UI_TEXT_FONT_SIZE,
                                    color: UI_NUMBER_TEXT_COLOR,
                                    ..default()
                                },
                            ),
                        ])
                        .with_style(Style {
                            width: Val::Percent(20.0),
                            margin: UiRect::new(
                                Val::Percent(0.0),
                                Val::Percent(5.0),
                                Val::Percent(0.0),
                                Val::Percent(3.8),
                            ),
                            ..default()
                        }),
                        RoundNumberText,
                        InGameHudUiElement,
                    ));
                });
        });
}

pub fn spawn_in_between_rounds_text(mut commands: Commands) {
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_section(
            "You are currently in between rounds.\nPress 'Space' to start the next round",
            TextStyle {
                font_size: 60.0,
                color: Color::BLACK,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            margin: UiRect::bottom(Val::Percent(32.5)),
            ..default()
        })
        .with_text_justify(JustifyText::Center),
        InBetweenRoundsHudUiElement,
    ));
}
