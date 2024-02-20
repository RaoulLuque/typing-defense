use super::*;

use bevy::hierarchy::Children;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;
use rand::Rng;

// Chance of spawning an enemy every super::resources::ENEMY_SPAWN_TIME seconds
pub const CHANCE_OF_SPAWNING_ENEMY: f64 = 1.0;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Words {
    pub vec_of_words: Vec<String>,
}

pub fn randomly_spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut last_enemy_spawn_point: ResMut<LastEnemySpawnPoint>,
    mut number_of_enemies: ResMut<NumberOfEnemies>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    words_handle: Res<WordsHandle>,
    words: Res<Assets<Words>>,
) {
    // Cap out enemies
    if number_of_enemies.number < 15 {
        // Get thread rng once for better performance
        let mut rng = rand::thread_rng();
        if enemy_spawn_timer.timer.finished() && rng.gen_bool(CHANCE_OF_SPAWNING_ENEMY) {
            let window = window_query.get_single().unwrap();

            // Get a random spawn point
            let spawn_point = last_enemy_spawn_point
                .spawn_point
                .next_spawn_point_excluding_self(&mut rng);
            last_enemy_spawn_point.spawn_point = spawn_point;
            let spawn_point_transform =
                generate_spawn_point_transform_from_enum(spawn_point, window);

            if let Some(word) = words.get(words_handle.0.id()) {
                let word_for_enemy = word
                    .vec_of_words
                    .choose(&mut rng)
                    .expect("The list of words shouldn't be empty");
                commands
                    .spawn((
                        SpriteBundle {
                            transform: spawn_point_transform,
                            texture: asset_server.load("sprites/skull.png"),
                            ..default()
                        },
                        Enemy {},
                        spawn_point,
                        Speed { speed: 100.0 },
                    ))
                    .with_children(|parent| {
                        parent.spawn(Text2dBundle {
                            text: Text {
                                sections: turn_string_literal_into_vec_of_text_sections(
                                    word_for_enemy,
                                ),
                                alignment: TextAlignment::Center,
                                linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                            },
                            // ensure the text is drawn on top of the box
                            transform: Transform::from_xyz(0.0, 50.0, 0.0),
                            ..default()
                        });
                    });
            }
            number_of_enemies.number += 1;
        }
    }
}

fn generate_spawn_point_transform_from_enum(
    enemy_spawn_point_enum: EnemySpawnPoint,
    window: &Window,
) -> Transform {
    match enemy_spawn_point_enum {
        EnemySpawnPoint::Top => Transform::from_xyz(0.0, window.height() * 0.5, 0.0),
        EnemySpawnPoint::Bottom => Transform::from_xyz(0.0, -window.height() * 0.5, 0.0),
        EnemySpawnPoint::Left => Transform::from_xyz(-window.width() * 0.5, 0.0, 0.0),
        EnemySpawnPoint::Right => Transform::from_xyz(window.width() * 0.5, 0.0, 0.0),
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn update_position_of_enemies(
    mut enemy_query: Query<(&Speed, &EnemySpawnPoint, &mut Transform), With<Enemy>>,
    time: Res<Time>,
) {
    for (speed, spawn_point, mut transform) in enemy_query.iter_mut() {
        let translation = match spawn_point {
            EnemySpawnPoint::Top => Vec3::new(0.0, -speed.speed * time.delta_seconds(), 0.0),
            EnemySpawnPoint::Bottom => Vec3::new(0.0, speed.speed * time.delta_seconds(), 0.0),
            EnemySpawnPoint::Left => Vec3::new(speed.speed * time.delta_seconds(), 0.0, 0.0),
            EnemySpawnPoint::Right => Vec3::new(-speed.speed * time.delta_seconds(), 0.0, 0.0),
        };
        transform.translation += translation;
    }
}

pub fn enemy_collision_with_castle(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut number_of_enemies: ResMut<NumberOfEnemies>,
) {
    for (entity, transform) in enemy_query.iter_mut() {
        if transform.translation.distance(Vec3::new(0.0, 0.0, 0.0)) < 5.0 {
            commands.entity(entity).despawn_recursive();
            number_of_enemies.number -= 1;
        }
    }
}

pub fn update_text_from_enemies_on_button_press(
    mut commands: Commands,
    mut enemies_being_typed: ResMut<EnemiesBeingTyped>,
    mut number_of_enemies: ResMut<NumberOfEnemies>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut q_parent: Query<(Entity, Option<&mut CurrentlyBeingTyped>, &Children), With<Enemy>>,
    mut q_child: Query<&mut Text>,
) {
    for key_event in keyboard_input_events.read() {
        if key_event.state != ButtonState::Pressed {
            continue;
        } else {
            // Case where key is being pressed
            if let Some(pressed_key) = key_event.key_code {
                // Check if esc or backspace was just pressed and reset all enemies if so
                if pressed_key == KeyCode::Escape || pressed_key == KeyCode::Back {
                    for (entity_id, currently_being_typed, child) in q_parent.iter_mut() {
                        if let Some(_) = currently_being_typed {
                            let mut iter = q_child.iter_many_mut(child);
                            while let Some(mut text) = iter.fetch_next() {
                                for section in text.sections.iter_mut() {
                                    section.style.color = Color::WHITE;
                                }
                            }
                            commands.entity(entity_id).remove::<CurrentlyBeingTyped>();
                        }
                    }
                    enemies_being_typed.indicator = false;
                    enemies_being_typed.vec_of_enemies.clear();
                }

                // Check if the key is a key and not a function/logical key otherwise can ignore
                if let Some(pressed_letter) = key_to_letter(pressed_key) {
                    let mut made_a_mistake_global = false;
                    // Iterate over all enemies with children and get typing index if necessary
                    for (entity_id, currently_being_typed, child) in q_parent.iter_mut() {
                        if !enemies_being_typed.indicator && !made_a_mistake_global {
                            // If nothing is currently being typed
                            let mut iter = q_child.iter_many_mut(child);
                            while let Some(mut text) = iter.fetch_next() {
                                let number_of_letter_in_word = text.sections.len();
                                if let Some(text_section) = text.sections.get_mut(0) {
                                    if text_section.value == pressed_letter {
                                        if number_of_letter_in_word == 1 {
                                            // Enemy only consists of one letter - You got "typed"
                                            // Despawn entity and remove entity from list of enemies that are currently being typed
                                            commands.entity(entity_id).despawn_recursive();
                                            number_of_enemies.number -= 1;
                                        } else {
                                            // Player is starting to type this enemy
                                            text_section.style.color = Color::ORANGE_RED;
                                            // Insert the currently being typed component into enemy
                                            commands
                                                .entity(entity_id)
                                                .insert(CurrentlyBeingTyped { index: 0 });
                                            enemies_being_typed.vec_of_enemies.push(entity_id);
                                        }
                                    }
                                }
                            }
                        } else {
                            // Something is being typed already
                            if let Some(mut currently_being_typed) = currently_being_typed {
                                let mut iter = q_child.iter_many_mut(child);
                                while let Some(mut text) = iter.fetch_next() {
                                    // Track if there is a mistake
                                    let mut made_a_mistake = false;
                                    if let Some(text_section) =
                                        text.sections.get_mut(currently_being_typed.index + 1)
                                    {
                                        if text_section.value == pressed_letter {
                                            // Player is continuing to type this enemy
                                            text_section.style.color = Color::ORANGE_RED;
                                            currently_being_typed.index =
                                                currently_being_typed.index + 1;
                                            if currently_being_typed.index
                                                == text.sections.len() - 1
                                            {
                                                // You got "typed"
                                                // Despawn entity and remove entity from list of enemies that are currently being typed
                                                commands.entity(entity_id).despawn_recursive();
                                                enemies_being_typed
                                                    .vec_of_enemies
                                                    .retain(|&x| x != entity_id);
                                                if enemies_being_typed.vec_of_enemies.len() == 0 {
                                                    // Check if there are no more enemies being typed
                                                    enemies_being_typed.indicator = false;
                                                }
                                                number_of_enemies.number -= 1;
                                            }
                                        } else {
                                            // Player is typing another enemy or has made a mistake
                                            made_a_mistake = true;
                                            made_a_mistake_global = true;
                                        }
                                    }
                                    if made_a_mistake {
                                        for section in text.sections.iter_mut() {
                                            section.style.color = Color::WHITE;
                                        }
                                        commands.entity(entity_id).remove::<CurrentlyBeingTyped>();
                                        enemies_being_typed
                                            .vec_of_enemies
                                            .retain(|&x| x != entity_id);
                                    }
                                    // If there were mistakes and there is no enemy left that is being typed
                                    if enemies_being_typed.vec_of_enemies.len() == 0 {
                                        enemies_being_typed.indicator = false;
                                    }
                                }
                            }
                        }
                    }
                    // Case where there were no enemies being typed before but now there is one
                    // This is done outside of the for loop in order not to exclude partial matches
                    if !enemies_being_typed.indicator
                        && enemies_being_typed.vec_of_enemies.len() > 0
                    {
                        // Set global resource that something is being typed accordingly
                        enemies_being_typed.indicator = true;
                    }
                }
            }
        }
    }
}

// Maps keys to letters and returns none if the key is not needed
fn key_to_letter(key: KeyCode) -> Option<String> {
    match key {
        KeyCode::A => Some("a".to_string()),
        KeyCode::B => Some("b".to_string()),
        KeyCode::C => Some("c".to_string()),
        KeyCode::D => Some("d".to_string()),
        KeyCode::E => Some("e".to_string()),
        KeyCode::F => Some("f".to_string()),
        KeyCode::G => Some("g".to_string()),
        KeyCode::H => Some("h".to_string()),
        KeyCode::I => Some("i".to_string()),
        KeyCode::J => Some("j".to_string()),
        KeyCode::K => Some("k".to_string()),
        KeyCode::L => Some("l".to_string()),
        KeyCode::M => Some("m".to_string()),
        KeyCode::N => Some("n".to_string()),
        KeyCode::O => Some("o".to_string()),
        KeyCode::P => Some("p".to_string()),
        KeyCode::Q => Some("q".to_string()),
        KeyCode::R => Some("r".to_string()),
        KeyCode::S => Some("s".to_string()),
        KeyCode::T => Some("t".to_string()),
        KeyCode::U => Some("u".to_string()),
        KeyCode::V => Some("v".to_string()),
        KeyCode::W => Some("w".to_string()),
        KeyCode::X => Some("x".to_string()),
        KeyCode::Y => Some("y".to_string()),
        KeyCode::Z => Some("z".to_string()),
        KeyCode::Apostrophe => Some("'".to_string()),
        _ => None,
    }
}

// Turns a string literal into a vector of text sections each containing one character from the string literal
fn turn_string_literal_into_vec_of_text_sections(string_literal: &str) -> Vec<TextSection> {
    string_literal
        .chars()
        .map(|x| {
            TextSection::new(
                x.to_string(),
                TextStyle {
                    font_size: 60.0,
                    ..default()
                },
            )
        })
        .collect()
}

pub fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    let words_handle =
        WordsHandle(asset_server.load("words/thousand_most_frequent_words.words.toml"));
    commands.insert_resource(words_handle);
}
