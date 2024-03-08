use bevy::app::AppExit;

use super::*;
use crate::game::RoundState;

pub fn setup_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::Main);
    println!("You are in the menu!");
}

pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                            UiImage::new(asset_server.load("menu/mainMenuBanner.png")),
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
                                            asset_server.load("menu/mainMenuButton.png"),
                                        ),
                                        ..default()
                                    },
                                    MenuButtonAction::Play,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text::from_section(
                                            "Start Game",
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
                                            asset_server.load("menu/mainMenuButton.png"),
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
    mut app_exit_events: EventWriter<AppExit>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<AppState>>,
    mut next_round_state: ResMut<NextState<RoundState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
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
        (Changed<Interaction>, With<Button>),
    >,
    asset_server: Res<AssetServer>,
) {
    for (interaction, mut ui_image) in &mut interaction_query {
        *ui_image = match *interaction {
            Interaction::Pressed => {
                UiImage::new(asset_server.load("menu/mainMenuButtonPressed.png"))
            }
            Interaction::Hovered => UiImage::new(asset_server.load("menu/mainMenuButtonHover.png")),
            Interaction::None => UiImage::new(asset_server.load("menu/mainMenuButton.png")),
        }
    }
}
