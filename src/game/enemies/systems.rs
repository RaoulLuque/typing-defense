use super::*;

use crate::game::rounds::resources::*;

use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;
use bevy::render::texture;
use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;
use rand::Rng;

// Chance of spawning an enemy every super::resources::ENEMY_SPAWN_TIME seconds
pub const CHANCE_OF_SPAWNING_ENEMY: f64 = 1.0;
// Base value which is divided by the enemy speed to get the animation speed - lower = faster
pub const BASE_ANIMATION_SPEED: f32 = 3.0;
// Scale factor by which enemy sprites are scaled - higher = bigger
pub const ENEMY_SPRITE_SCALE_FACTOR: f32 = 1.4;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Words {
    pub vec_of_words: Vec<String>,
}

pub fn randomly_spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut last_enemy_spawn_point: ResMut<LastEnemySpawnPoint>,
    mut number_of_enemies_spawned_this_round: ResMut<NumberOfEnemiesSpawnedCurrentRound>,
    max_number_of_enemies_this_round: Res<MaxNumberOfEnemiesCurrentRound>,
    enemy_base_speed_this_round: Res<EnemyBaseSpeedCurrentRound>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    words_handle: Res<WordsHandle>,
    words: Res<Assets<Words>>,
) {
    // Spawn only as many enemies as is planned for this round
    if number_of_enemies_spawned_this_round.number < max_number_of_enemies_this_round.number {
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

            // Get random enemy sprite
            let enemy_type: EnemyType = rng.gen();
            let (enemy_name, sprite_width, sprite_height, animation_length) =
                pick_random_enemy_and_generate_sprite_information(&enemy_type);
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
            let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);
            // Set speed of enemy randomly in range of 0.625 to 1.375 times the enemy base speed this round
            let speed = (rng.gen::<f32>() * 0.75 + 0.625) * enemy_base_speed_this_round.speed;
            let walking_animation: WalkingAnimation = WalkingAnimation {
                length_of_animation: animation_length,
                animation_timer: Timer::from_seconds(
                    BASE_ANIMATION_SPEED / speed,
                    TimerMode::Repeating,
                ),
            };

            // Flip the sprite on the y-axis if enemy is spawned left or bottom
            let flip_on_y_axis = match spawn_point {
                EnemySpawnPoint::Left => true,
                EnemySpawnPoint::Bottom => rng.gen_bool(0.5),
                EnemySpawnPoint::Top => rng.gen_bool(0.5),
                EnemySpawnPoint::Right => false,
            };

            // Resize the sprites for game
            let custom_sprite_size = Some(Vec2::new(
                ENEMY_SPRITE_SCALE_FACTOR * sprite_width,
                ENEMY_SPRITE_SCALE_FACTOR * sprite_height,
            ));

            if let Some(word) = words.get(words_handle.0.id()) {
                // Get random word from list
                let word_for_enemy = word
                    .vec_of_words
                    .choose(&mut rng)
                    .expect("The list of words shouldn't be empty");
                commands
                    .spawn((
                        SpriteSheetBundle {
                            transform: spawn_point_transform,
                            sprite: TextureAtlasSprite {
                                flip_x: flip_on_y_axis,
                                index: 0,
                                custom_size: custom_sprite_size,
                                ..default()
                            },
                            texture_atlas: texture_atlas_handle,
                            ..default()
                        },
                        Enemy {},
                        spawn_point,
                        Speed { speed: speed },
                        walking_animation,
                        enemy_type,
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
            number_of_enemies_spawned_this_round.number += 1;
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

/// Sets the
pub fn pick_random_enemy_and_generate_sprite_information(
    enemy_type: &EnemyType,
) -> (
    // Path for sprite
    String,
    // Sprite width
    f32,
    // Sprite height
    f32,
    // Number of sprite in the spritesheet/animation
    usize,
) {
    match enemy_type {
        EnemyType::Pig => ("pig".to_string(), 36.0, 30.0, 16),
        EnemyType::Bat => ("bat".to_string(), 46.0, 30.0, 7),
        EnemyType::Bee => ("bee".to_string(), 36.0, 34.0, 6),
        EnemyType::Bunny => ("bunny".to_string(), 34.0, 44.0, 12),
        EnemyType::Chicken => ("chicken".to_string(), 32.0, 34.0, 14),
        EnemyType::Mushroom => ("mushroom".to_string(), 32.0, 32.0, 16),
        EnemyType::Trunk => ("trunk".to_string(), 64.0, 32.0, 14),
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn animate_enemies(
    time: Res<Time>,
    mut enemy_query: Query<(&mut WalkingAnimation, &mut TextureAtlasSprite)>,
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
    castle_query: Query<&Transform, With<castle::components::Castle>>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedCurrentRound>,
    mut number_of_lives_left: ResMut<castle::resources::NumberOfLivesLeft>,
) {
    if let Ok(castle_transform) = castle_query.get_single() {
        for (entity, transform) in enemy_query.iter_mut() {
            if transform.translation.distance(castle_transform.translation) < 5.0 {
                commands.entity(entity).despawn_recursive();
                number_of_enemies_typed_current_round.number += 1;
                number_of_lives_left.number -= 1;
            }
        }
    }
}

pub fn despawn_enemy_if_out_of_screen(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedCurrentRound>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().expect("Window should exist");
    for (enemy_entity, enemy_transform) in &enemy_query {
        if enemy_transform.translation.x > window.width() * 0.7
            || enemy_transform.translation.x < -window.width() * 0.7
            || enemy_transform.translation.y > window.height() * 0.7
            || enemy_transform.translation.y < -window.height() * 0.7
        {
            commands.entity(enemy_entity).despawn_recursive();
            number_of_enemies_typed_current_round.number += 1;
        }
    }
}

pub fn update_text_from_enemies_on_button_press(
    mut commands: Commands,
    mut enemies_being_typed: ResMut<EnemiesBeingTyped>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedCurrentRound>,
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
                                            // You got "typed"
                                            // Enemy only consists of one letter - You got "typed"
                                            // Despawn entity and remove entity from list of enemies that are currently being typed
                                            commands.entity(entity_id).despawn_recursive();
                                            number_of_enemies_typed_current_round.number += 1;
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
                                                number_of_enemies_typed_current_round.number += 1;
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

pub fn setup_list_of_words_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let words_handle =
        WordsHandle(asset_server.load("words/thousand_most_frequent_words.words.toml"));
    commands.insert_resource(words_handle);
}
