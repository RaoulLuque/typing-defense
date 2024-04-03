use rand::seq::SliceRandom;

use super::*;
use crate::game::{
    enemies::{
        components::{Enemy, EnemyType, Speed, WalkingAnimation},
        resources::WordsHandle,
        systems::{
            generate_sprite_information_from_enemy_type,
            turn_string_literal_into_vec_of_text_sections, Words, BASE_ANIMATION_SPEED,
            STANDARD_TEXT_COLOR,
        },
    },
    rounds_and_indicators::resources::{Difficulty, DifficultyIndicator, INITIAL_ENEMY_SPEED},
    RoundState,
};

#[derive(Event)]
pub struct DifficultyChangedEvent(bool);

#[derive(Event)]
pub struct Restart;

pub enum MenuType {
    MainMenu,
    InGameMenu,
    LostMenu,
}

const BUTTON_HEIGHT: f32 = 15.0;

pub fn setup_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::Main);
}

pub fn spawn_main_menu(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_menu(commands, asset_server, MenuType::MainMenu);
}

fn spawn_menu(mut commands: Commands, asset_server: Res<AssetServer>, type_of_menu: MenuType) {
    let button_text_style = TextStyle {
        font_size: 40.0,
        ..default()
    };

    let game_lost_text_style = TextStyle {
        font_size: 22.0,
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
                                    width: Val::Percent(65.0),
                                    height: Val::Percent(110.0),
                                    align_items: AlignItems::Center,
                                    flex_direction: FlexDirection::Column,
                                    row_gap: Val::Percent(2.0),
                                    ..default()
                                },
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            UiImage::new(asset_server.load("ui/menu/mainMenuBanner.png")),
                        ))
                        .with_children(|parent| {

                            match type_of_menu {
                                MenuType::LostMenu => {
                                    parent
                                        .spawn(
                                    TextBundle {
                                        text: Text::from_section(
                                            "Unfortunately you've lost. You may continue playing without increasing your score or restart the game. You can restart anytime whilst continuing by going to the menu with 'esc'",
                                            game_lost_text_style.clone(),
                                        ),
                                        style: Style {
                                            margin: UiRect {
                                                top: Val::Percent(22.0),
                                                bottom: Val::Percent(4.0),
                                                left: Val::Percent(0.0),
                                                right: Val::Percent(0.0),
                                            },
                                            width: Val::Percent(30.0),
                                            height: Val::Percent(10.0),
                                            ..default()
                                        },
                                        ..default()
                                    });
                                }
                                MenuType::InGameMenu | MenuType::MainMenu => {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: Style {
                                                    width: Val::Percent(45.0),
                                                    height: Val::Percent(BUTTON_HEIGHT),
                                                    align_items: AlignItems::Center,
                                                    justify_content: JustifyContent::Center,
                                                    flex_direction: FlexDirection::Column,
                                                    margin: UiRect::top(Val::VMin(25.0)),
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
                                                MenuType::InGameMenu | _ => {
                                                    MenuButtonAction::Resume
                                                }
                                            },
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn((
                                                TextBundle {
                                                    text: Text::from_section(
                                                        match type_of_menu {
                                                            MenuType::MainMenu => "Start Game",
                                                            MenuType::InGameMenu | _ => {
                                                                "Resume Game"
                                                            }
                                                        },
                                                        button_text_style.clone(),
                                                    ),
                                                    style: Style {
                                                        margin: UiRect::bottom(Val::Percent(5.0)),
                                                        ..default()
                                                    },
                                                    ..default()
                                                },
                                                MainMenuText,
                                            ));
                                        });
                                }
                            }

                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(45.0),
                                            height: Val::Percent(BUTTON_HEIGHT),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        image: UiImage::new(
                                            asset_server.load("ui/menu/mainMenuButton.png"),
                                        ),
                                        ..default()
                                    },
                                    match type_of_menu {
                                        MenuType::MainMenu | MenuType::InGameMenu => MenuButtonAction::HowToPlay,
                                         MenuType::LostMenu => {
                                            MenuButtonAction::Resume
                                        }
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle {
                                            text: Text::from_section(
                                                match type_of_menu {
                                                    MenuType::MainMenu | MenuType::InGameMenu => "How to play",
                                                    MenuType::LostMenu => {
                                                        "Continue"
                                                    }
                                                },
                                                button_text_style.clone(),
                                            ),
                                            style: Style {
                                                margin: UiRect::bottom(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        MainMenuText,
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: Style {
                                            width: Val::Percent(45.0),
                                            height: Val::Percent(BUTTON_HEIGHT),
                                            align_items: AlignItems::Center,
                                            justify_content: JustifyContent::Center,
                                            flex_direction: FlexDirection::Column,
                                            ..default()
                                        },
                                        background_color: Color::WHITE.into(),
                                        image: UiImage::new(
                                            asset_server.load("ui/menu/mainMenuButton.png"),
                                        ),
                                        ..default()
                                    },
                                    match type_of_menu {
                                        MenuType::MainMenu => MenuButtonAction::Exit,
                                        MenuType::InGameMenu | MenuType::LostMenu => {
                                            MenuButtonAction::Restart
                                        }
                                    },
                                ))
                                .with_children(|parent| {
                                    parent.spawn((
                                        TextBundle {
                                            text: Text::from_section(
                                                match type_of_menu {
                                                    MenuType::MainMenu => "Exit",
                                                    MenuType::InGameMenu | MenuType::LostMenu => {
                                                        "Restart"
                                                    }
                                                },
                                                button_text_style.clone(),
                                            ),
                                            style: Style {
                                                margin: UiRect::bottom(Val::Percent(5.0)),
                                                ..default()
                                            },
                                            ..default()
                                        },
                                        MainMenuText,
                                    ));
                                });
                        });
                });
        });
    commands.spawn((
        ButtonBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_self: AlignSelf::FlexStart,
                justify_self: JustifySelf::Start,
                width: Val::Percent(10.0),
                aspect_ratio: Some(230.0 / 225.0),
                // Vertical align of menu banner
                margin: UiRect {
                    left: Val::Percent(2.),
                    right: Val::Percent(0.),
                    top: Val::Percent(2.),
                    bottom: Val::Percent(0.),
                },
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: Color::WHITE.into(),
            image: UiImage::new(asset_server.load("icon/github.png")),
            ..default()
        },
        MenuButtonAction::OpenLink("https://github.com/RaoulLuque/typing-defense".to_string()),
        MainMenuScreenUiElement,
        GitHubButton,
    ));
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
pub fn despawn_entities_with_specific_component<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
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
    game_started_state: Res<State<GameStartedState>>,
    mut next_game_started_state: ResMut<NextState<GameStartedState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
    mut restart_event_writer: EventWriter<Restart>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Resume => {
                    simulation_state_next_state.set(SimulationState::Running);
                    next_menu_state.set(MenuState::NotInTheMenu);
                }
                MenuButtonAction::Play => {
                    next_game_state.set(AppState::InGame);
                    next_menu_state.set(MenuState::NotInTheMenu);
                    next_round_state.set(RoundState::InRound);
                    next_game_started_state.set(GameStartedState::GameHasStarted);
                }
                MenuButtonAction::HowToPlay => {
                    next_menu_state.set(MenuState::HowToPlayTransition);
                }
                MenuButtonAction::Main => {
                    next_menu_state.set(MenuState::Main);
                }
                MenuButtonAction::OpenLink(link) => {
                    if let Err(error) = webbrowser::open(link) {
                        warn!("Failed to open link {error:?}");
                    }
                }
                MenuButtonAction::Back => match game_started_state.get() {
                    GameStartedState::GameHasNotStarted => {
                        next_menu_state.set(MenuState::Main);
                    }
                    GameStartedState::GameHasStarted => {
                        next_menu_state.set(MenuState::InGameMainMenu);
                    }
                },
                MenuButtonAction::Exit => {
                    app_exit_events.send(bevy::app::AppExit);
                }
                MenuButtonAction::Restart => {
                    restart_event_writer.send(Restart);
                }
            }
        }
    }
}

pub fn check_if_in_game_menu_is_opened(
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut simulation_state_next_state: ResMut<NextState<SimulationState>>,
    mut menu_state_next_state: ResMut<NextState<MenuState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        if simulation_state.get() == &SimulationState::Running {
            simulation_state_next_state.set(SimulationState::Paused);
            menu_state_next_state.set(MenuState::InGameMainMenu);
        };
    }
}

pub fn spawn_in_game_menu(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_menu(commands, asset_server, MenuType::InGameMenu);
}

pub fn spawn_lost_menu(commands: Commands, asset_server: Res<AssetServer>) {
    spawn_menu(commands, asset_server, MenuType::LostMenu);
}

// This system handles changing all buttons color based on mouse interaction
pub fn menu_button_animations(
    mut interaction_query: Query<
        (&Interaction, &mut UiImage, &Children),
        (Changed<Interaction>, With<Button>, With<MenuButtonAction>),
    >,
    mut text_query: Query<&mut Style, (With<Text>, With<MainMenuText>)>,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut ui_image, children) in &mut interaction_query {
        *ui_image = match *interaction {
            Interaction::Pressed => {
                let mut text_iter = text_query.iter_many_mut(children);
                if let Some(mut text_style) = text_iter.fetch_next() {
                    text_style.margin.bottom = Val::Percent(2.5);
                };
                UiImage::new(asset_server.load("ui/menu/mainMenuButtonPressed.png"))
            }
            Interaction::Hovered => {
                let mut text_iter = text_query.iter_many_mut(children);
                if let Some(mut text_style) = text_iter.fetch_next() {
                    text_style.margin.bottom = Val::Percent(2.5);
                };
                UiImage::new(asset_server.load("ui/menu/mainMenuButtonPressed.png"))
            }
            Interaction::None => {
                let mut text_iter = text_query.iter_many_mut(children);
                if let Some(mut text_style) = text_iter.fetch_next() {
                    text_style.margin.bottom = Val::Percent(5.0);
                };
                UiImage::new(asset_server.load("ui/menu/mainMenuButton.png"))
            }
        }
    }
}

pub fn github_button_animation(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<GitHubButton>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::None => {
                *color = Color::WHITE.into();
            }
            _ => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
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

pub fn spawn_how_to_play_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    words_handle: Res<WordsHandle>,
    words: Res<Assets<Words>>,
) {
    let how_to_play_text_style = TextStyle {
        font_size: 22.0,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 20.0,
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
            HowToPlayScreenUiElement,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(65.0),
                            height: Val::Percent(120.0),
                            // Vertical align of menu banner
                            margin: UiRect::top(Val::VMin(10.)),
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::Column,
                            row_gap: Val::Percent(1.5),
                            ..default()
                        },
                        background_color: Color::WHITE.into(),
                        ..default()
                    },
                    UiImage::new(asset_server.load("ui/menu/mainMenuBanner.png")),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle {
                            text: Text::from_section(
                                "The goal of the game is to prevent the animals that spawn at the borders of the screen from reaching the castle in the middle. This is done by typing the words that are above the animals. Try 'typing' the animal below:",
                                how_to_play_text_style.clone(),
                            ),
                            style: Style {
                                margin: UiRect::top(Val::Percent(24.0)),
                                width: Val::Percent(30.0),
                                height: Val::Percent(13.5),
                                ..default()
                            },
                            ..default()
                        },
                    ));

                    let mut rng = rand::thread_rng();

                    // Get random enemy sprite
                    let enemy_type: EnemyType = EnemyType::Mushroom;
                    let (enemy_name, sprite_width, sprite_height, animation_length) =
                        generate_sprite_information_from_enemy_type(&enemy_type);
                    let texture_handle: Handle<Image> =
                        asset_server.load(format!("sprites/enemies/{}.png", enemy_name));
                    let texture_atlas = TextureAtlas::from_grid(
                        texture_handle,
                        Vec2::new(sprite_width, sprite_height),
                        animation_length,
                        1,
                        None,
                        None,
                    );
                    let texture_atlas_handle: Handle<TextureAtlas> =
                        texture_atlases.add(texture_atlas);

                    // Set speed of enemy randomly in range of 0.625 to 1.375 times the enemy base speed this round
                    let walking_animation: WalkingAnimation = WalkingAnimation {
                        length_of_animation: animation_length,
                        animation_timer: Timer::from_seconds(
                            BASE_ANIMATION_SPEED / INITIAL_ENEMY_SPEED,
                            TimerMode::Repeating,
                        ),
                    };

                    let word = words
                        .get(words_handle.0.id())
                        .expect("Word list shouldn't be empty");
                    // Get random word from list
                    let word_for_enemy = word
                        .vec_of_words
                        .choose(&mut rng)
                        .expect("The list of words shouldn't be empty");

                    parent.spawn( NodeBundle {
                        style: Style {
                            height: Val::Percent(14.0),
                            aspect_ratio: Some(sprite_width / sprite_height),
                            ..default()
                        },
                        ..default()
                    }).with_children(|parent| {
                        parent.spawn((
                            AtlasImageBundle {
                                style: Style {
                                    height: Val::Percent(100.0),
                                    width: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::FlexStart,
                                    // flex_grow: 0.5,
                                    ..default()
                                },
                                texture_atlas_image: UiTextureAtlasImage {
                                    flip_x: true,
                                    index: 0,
                                    ..default()
                                },
                                texture_atlas: texture_atlas_handle,
                                ..default()
                            },
                            Enemy {},
                            Speed {
                                speed: INITIAL_ENEMY_SPEED,
                            },
                            walking_animation,
                            enemy_type,
                            Name::new(word_for_enemy.clone()),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                TextBundle {
                                    text: Text {
                                        sections: turn_string_literal_into_vec_of_text_sections(
                                            word_for_enemy,
                                            STANDARD_TEXT_COLOR,
                                        ),
                                        alignment: TextAlignment::Center,
                                        linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                                    },
                                    // ensure the text is drawn on top of the box
                                    style: Style {
                                        ..default()
                                    },
                                    ..default()
                                },
                            ));
                        });
                    });

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(30.0),
                                height: Val::Percent(5.0),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceEvenly,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            ..default()
                        },
                    )).with_children(|parent| {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(45.0),
                                        height: Val::Percent(100.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        flex_direction: FlexDirection::Row,
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
                                parent.spawn((
                                    TextBundle {
                                        text: Text::from_section(
                                            "Spawn",
                                            button_text_style.clone(),
                                        ),
                                        style: Style {
                                            margin: UiRect::bottom(Val::Percent(5.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    MainMenuText,
                                ));
                            });

                            parent
                            .spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(45.0),
                                        height: Val::Percent(100.0),
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::SpaceEvenly,
                                        flex_direction: FlexDirection::Row,
                                        ..default()
                                    },
                                    background_color: Color::WHITE.into(),
                                    image: UiImage::new(
                                        asset_server.load("ui/menu/mainMenuButton.png"),
                                    ),
                                    ..default()
                                },
                                MenuButtonAction::Back,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle {
                                        text: Text::from_section(
                                            "Back",
                                            button_text_style.clone(),
                                        ),
                                        style: Style {
                                            margin: UiRect::bottom(Val::Percent(5.0)),
                                            ..default()
                                        },
                                        ..default()
                                    },
                                    MainMenuText,
                                ));
                            });
                        });
                        parent.spawn((
                            TextBundle {
                                text: Text::from_section(
                                    "Hint: Try using backspace or intentionally typing wrong letters\nAlso: Esc Pauses the game",
                                    how_to_play_text_style.clone(),
                                ),
                                style: Style {
                                    width: Val::Percent(30.0),
                                    ..default()
                                },
                                ..default()
                            },
                        ));
                });
        });
}

pub fn animate_enemies_in_how_to_play(
    time: Res<Time>,
    mut enemy_query: Query<(&mut WalkingAnimation, &mut UiTextureAtlasImage)>,
) {
    for (mut walking_animation, mut atlas_sprite) in &mut enemy_query {
        walking_animation.animation_timer.tick(time.delta());
        if walking_animation.animation_timer.just_finished() {
            atlas_sprite.index = if atlas_sprite.index == walking_animation.length_of_animation - 1
            {
                0
            } else {
                atlas_sprite.index + 1
            };
        }
    }
}

pub fn transition_to_how_to_play(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::HowToPlay);
}
