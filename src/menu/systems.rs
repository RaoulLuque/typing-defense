use bevy::{app::AppExit, render::settings};

use super::*;
use crate::game::{
    rounds_and_indicators::resources::{Difficulty, DifficultyIndicator},
    RoundState, SimulationState,
};

#[derive(Event)]
pub struct DifficultyChangedEvent(bool);

pub enum MenuType {
    MainMenu,
    InGameMenu,
}

pub fn setup_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::Main);
    println!("You are in the menu!");
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    spawn_menu(commands, asset_server, MenuType::MainMenu);
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>, type_of_menu: MenuType) {
    let button_text_style = TextStyle {
        font_size: 40.0,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            MainMenuScreenUiElement,
        ))
        .with_children(|parent| {
            parent
                .spawn((NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexStart,
                        ..default()
                    },
                    ..default()
                },))
                .with_children(|parent| {
                    parent
                        .spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Percent(46.875),
                                    height: Val::Percent(54.68),
                                    // Vertical align of menu banner
                                    margin: UiRect::top(Val::VMin(25.)),
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Percent(4.0),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UiImage::new(asset_server.load("ui/menu/mainMenuBanner.png")),
                        ))
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(60.0),
                                            height: Val::Percent(25.0),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            flex_direction: FlexDirection::Column,
                                            margin: UiRect::top(Val::VMin(9.)),
                                            ..default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        image: UiImage::new(
                                            asset_server.load("ui/menu/mainMenuButton.png"),
                                        ),
                                        ..default()
                                    },
                                    match type_of_menu {
                                        MenuType::MainMenu => MenuButtonAction::Play,
                                        MenuType::InGameMenu => MenuButtonAction::Resume,
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            match type_of_menu {
                                                MenuType::MainMenu => "Start Game",
                                                MenuType::InGameMenu => "Resume Game",
                                            },
                                            button_text_style.clone(),
                                        ),
                                        style: Style {
                                            margin: UiRect::bottom(Val::Percent(5.0)),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(60.0),
                                            height: Val::Percent(25.0),
                                            align_items: AlignItems::Center,
                                            flex_direction: FlexDirection::Column,
                                            justify_content: JustifyContent::Center,
                                            ..default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        image: UiImage::new(
                                            asset_server.load("ui/menu/mainMenuButton.png"),
                                        ),
                                        ..default()
                                    },
                                    MenuButtonAction::HowToPlay,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "How to play",
                                            button_text_style.clone(),
                                        ),
                                        style: Style {
                                            margin: UiRect::bottom(Val::Percent(5.0)),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        });
                });
        });
}

pub fn transition_from_menu_to_in_game(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        if app_state.get() != &AppState::InGame {
            next_app_state.set(AppState::InGame);
            next_round_state.set(RoundState::InRound);
            next_menu_state.set(MenuState::NotInTheMenu);
        }
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

// Function for handling the buttons in the main menu
pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<AppState>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Resume => {
                    simulation_state_next_state.set(SimulationState::Running)
                }
                MenuButtonAction::Play => {
                    next_game_state.set(AppState::InGame);
                    next_menu_state.set(MenuState::NotInTheMenu);
                    next_round_state.set(RoundState::InRound);
                }
                MenuButtonAction::HowToPlay => {
                    next_menu_state.set(MenuState::HowToPlay);
                }
            }
        }
    }
}

// This system handles changing all buttons color based on mouse interaction
pub fn menu_button_animations(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage),
        (Changed<Interaction>, With<Button>, With<MenuButtonAction>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut ui_image) in &mut interaction_query {
        *ui_image = match *interaction {
            Interaction::Pressed => {
                UiImage::new(asset_server.load("ui/menu/mainMenuButtonPressed.png"))
            }
            Interaction::Hovered => {
                UiImage::new(asset_server.load("ui/menu/mainMenuButtonPressed.png"))
            }
            Interaction::None => UiImage::new(asset_server.load("ui/menu/mainMenuButton.png")),
        }
    }
}

/// Spawns the settings button at the top right of the screen
pub fn spawn_settings_button(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            SettingsMenuClosed,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(8.0),
                            height: Val::Auto,
                            aspect_ratio: Some(1.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("ui/settings/settings_wheel_background.png")),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Percent(50.0),
                                height: Val::Percent(50.0),
                                margin: UiRect::bottom(Val::Percent(10.0)),
                                ..default()
                            },
                            image: UiImage::new(
                                asset_server.load("ui/settings/settings_wheel.png"),
                            ),
                            ..default()
                        },
                        SettingsButton::OpenSettings,
                    ));
                });
        });
}

/// Spawns the settings menu at the top right of the screen
pub fn spawn_settings_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    difficulty: Res<DifficultyIndicator>,
) {
    let settings_text_style = TextStyle {
        font_size: 30.0,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    ..default()
                },
                ..default()
            },
            SettingsMenuOpened,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(20.0),
                            height: Val::Auto,
                            aspect_ratio: Some(2.5),
                            flex_direction: FlexDirection::Column,
                            margin: UiRect::top(Val::Percent(1.0)),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("ui/settings/settings_background.png")),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                width: Val::Percent(80.0),
                                height: Val::Percent(40.0),
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section("Difficulty", settings_text_style.clone()),
                                style: Style {
                                    margin: UiRect::new(
                                        Val::Percent(10.0),
                                        Val::Percent(0.0),
                                        Val::Percent(0.0),
                                        Val::Percent(2.5),
                                    ),
                                    ..default()
                                },
                                ..default()
                            });
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height: Val::Percent(70.0),
                                        width: Val::Auto,
                                        aspect_ratio: Some(1.0),
                                        // margin: UiRect::right(Val::Percent(25.0)),
                                        ..default()
                                    },
                                    image: UiImage::new(asset_server.load("ui/settings/close.png")),
                                    ..default()
                                },
                                SettingsButton::CloseSettings,
                            ));
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn((NodeBundle {
                            style: Style {
                                width: Val::Percent(75.0),
                                height: Val::Percent(60.0),
                                justify_self: JustifySelf::Start,
                                justify_content: JustifyContent::SpaceEvenly,
                                align_items: AlignItems::FlexStart,
                                ..default()
                            },
                            ..default()
                        },))
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height: Val::Percent(60.0),
                                        width: Val::Auto,
                                        aspect_ratio: Some(1.0),
                                        ..default()
                                    },
                                    image: match difficulty.difficulty {
                                        Difficulty::Easy => UiImage::new(
                                            asset_server.load("ui/settings/minus_pressed.png"),
                                        ),
                                        _ => {
                                            UiImage::new(asset_server.load("ui/settings/minus.png"))
                                        }
                                    },
                                    ..default()
                                },
                                SettingsButton::Minus,
                            ));
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text::from_section(
                                        match difficulty.difficulty {
                                            Difficulty::Easy => "Easy",
                                            Difficulty::Medium => "Medium",
                                            Difficulty::Hard => "Hard",
                                        },
                                        settings_text_style.clone(),
                                    ),
                                    style: Style {
                                        margin: UiRect::new(
                                            Val::Percent(0.0),
                                            Val::Percent(0.0),
                                            Val::Percent(0.0),
                                            Val::Percent(0.0),
                                        ),
                                        ..default()
                                    },
                                    ..default()
                                },
                                DifficultySettingsText {},
                            ));
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        height: Val::Percent(60.0),
                                        width: Val::Auto,
                                        aspect_ratio: Some(1.0),
                                        ..default()
                                    },
                                    image: match difficulty.difficulty {
                                        Difficulty::Hard => UiImage::new(
                                            asset_server.load("ui/settings/plus_pressed.png"),
                                        ),
                                        _ => {
                                            UiImage::new(asset_server.load("ui/settings/plus.png"))
                                        }
                                    },
                                    ..default()
                                },
                                SettingsButton::Plus,
                            ));
                        });
                });
        });
}

/// Handling the buttons in the settings menu
pub fn settings_action(
    interaction_query: Query<(&Interaction, &SettingsButton), (Changed<Interaction>, With<Button>)>,
    mut next_settings_state: ResMut<NextState<SettingsMenuState>>,
    mut difficulty_changed_event_writer: EventWriter<DifficultyChangedEvent>,
) {
    for (interaction, settings_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match settings_button_action {
                SettingsButton::OpenSettings => {
                    next_settings_state.set(SettingsMenuState::SettingsOpened)
                }
                SettingsButton::CloseSettings => {
                    next_settings_state.set(SettingsMenuState::SettingsClosed);
                }
                SettingsButton::Plus => {
                    difficulty_changed_event_writer.send(DifficultyChangedEvent(true));
                }
                SettingsButton::Minus => {
                    difficulty_changed_event_writer.send(DifficultyChangedEvent(false));
                }
            }
        }
    }
}

/// Handles changing settings button based on mouse interactions
pub fn settings_button_animations(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage, &SettingsButton),
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
    difficulty: Res<DifficultyIndicator>,
) {
    for (interaction, mut ui_image, settings_button) in &mut interaction_query {
        *ui_image = match (*interaction, settings_button) {
            (Interaction::Pressed, SettingsButton::OpenSettings) => {
                UiImage::new(asset_server.load("ui/settings/settings_wheel_pressed.png"))
            }
            (Interaction::Pressed, SettingsButton::CloseSettings) => {
                UiImage::new(asset_server.load("ui/settings/close_pressed.png"))
            }
            (Interaction::Hovered, SettingsButton::OpenSettings) => {
                UiImage::new(asset_server.load("ui/settings/settings_wheel_pressed.png"))
            }
            (Interaction::Hovered, SettingsButton::CloseSettings) => {
                UiImage::new(asset_server.load("ui/settings/close_pressed.png"))
            }
            (Interaction::Pressed, SettingsButton::Plus) => {
                UiImage::new(asset_server.load("ui/settings/plus_pressed.png"))
            }
            (Interaction::Pressed, SettingsButton::Minus) => {
                UiImage::new(asset_server.load("ui/settings/minus_pressed.png"))
            }
            (Interaction::Hovered, SettingsButton::Plus) => {
                UiImage::new(asset_server.load("ui/settings/plus_pressed.png"))
            }
            (Interaction::Hovered, SettingsButton::Minus) => {
                UiImage::new(asset_server.load("ui/settings/minus_pressed.png"))
            }
            (_, SettingsButton::OpenSettings) => {
                UiImage::new(asset_server.load("ui/settings/settings_wheel.png"))
            }
            (_, SettingsButton::CloseSettings) => {
                UiImage::new(asset_server.load("ui/settings/close.png"))
            }
            // Check if button needs to be "permanently pressed" in ui
            (_, SettingsButton::Minus) => match difficulty.difficulty {
                Difficulty::Easy => {
                    UiImage::new(asset_server.load("ui/settings/minus_pressed.png"))
                }
                _ => UiImage::new(asset_server.load("ui/settings/minus.png")),
            },
            (_, SettingsButton::Plus) => match difficulty.difficulty {
                Difficulty::Hard => UiImage::new(asset_server.load("ui/settings/plus_pressed.png")),
                _ => UiImage::new(asset_server.load("ui/settings/plus.png")),
            },
        }
    }
}

pub fn change_difficulty(
    mut difficulty_changed_event_reader: EventReader<DifficultyChangedEvent>,
    mut difficulty: ResMut<DifficultyIndicator>,
    mut query_text_in_settings_menu: Query<&mut Text, With<DifficultySettingsText>>,
    mut button_query: Query<(&mut UiImage, &SettingsButton), With<Button>>,
    asset_server: Res<AssetServer>,
) {
    for difficulty_changed_event in difficulty_changed_event_reader.read() {
        difficulty.difficulty = match (difficulty_changed_event.0, &difficulty.difficulty) {
            (true, Difficulty::Easy) => Difficulty::Medium,
            (true, Difficulty::Medium) => Difficulty::Hard,
            (true, Difficulty::Hard) => Difficulty::Hard,
            (false, Difficulty::Easy) => Difficulty::Easy,
            (false, Difficulty::Medium) => Difficulty::Easy,
            (false, Difficulty::Hard) => Difficulty::Medium,
        };
        for mut text in query_text_in_settings_menu.iter_mut() {
            text.sections[0].value = difficulty.difficulty.to_string();
        }

        // Check if button needs to be "unpressed" in ui
        for (mut ui_image, settings_button) in &mut button_query {
            if settings_button == &SettingsButton::Minus {
                if difficulty_changed_event.0 && difficulty.difficulty == Difficulty::Medium {
                    *ui_image = UiImage::new(asset_server.load("ui/settings/minus.png"));
                }
            } else if settings_button == &SettingsButton::Plus {
                if !difficulty_changed_event.0 && difficulty.difficulty == Difficulty::Medium {
                    *ui_image = UiImage::new(asset_server.load("ui/settings/plus.png"));
                }
            }
        }
    }
}

pub fn check_if_in_game_menu_is_opened(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match simulation_state.get() {
            &SimulationState::Running => {
                simulation_state_next_state.set(SimulationState::Paused);
                spawn_menu(commands, asset_server, MenuType::InGameMenu);
            }
            _ => (),
        };
    }
}
