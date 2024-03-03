use bevy::input::{keyboard::KeyboardInput, ButtonState};

use enemies::rounds::resources::NumberOfEnemiesTypedCurrentRound;

use self::enemies::movement::components::{EnemySpawnPoint, PathCheckpointNumber};

use super::*;

pub fn update_text_from_enemies_on_button_press(
    mut commands: Commands,
    mut enemies_being_typed: ResMut<EnemiesBeingTyped>,
    mut number_of_enemies_typed_current_round: ResMut<NumberOfEnemiesTypedCurrentRound>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut q_parent_with_enemy: Query<
        (Entity, Option<&mut CurrentlyBeingTyped>, &Children),
        With<Enemy>,
    >,
    mut q_child_with_text: Query<&mut Text>,
) {
    for key_event in keyboard_input_events.read() {
        if key_event.state != ButtonState::Pressed {
            continue;
        } else {
            // Case where key is being pressed
            if let Some(pressed_key) = key_event.key_code {
                // Check if esc or backspace was just pressed and reset all enemies if so
                if pressed_key == KeyCode::Escape || pressed_key == KeyCode::Back {
                    for (entity_id, currently_being_typed, child) in q_parent_with_enemy.iter_mut()
                    {
                        if let Some(_) = currently_being_typed {
                            let mut iter = q_child_with_text.iter_many_mut(child);
                            while let Some(mut text) = iter.fetch_next() {
                                for section in text.sections.iter_mut() {
                                    section.style.color = STANDARD_TEXT_COLOR;
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
                    for (entity_id, currently_being_typed, child) in q_parent_with_enemy.iter_mut()
                    {
                        if !enemies_being_typed.indicator && !made_a_mistake_global {
                            // If nothing is currently being typed
                            let mut iter = q_child_with_text.iter_many_mut(child);
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
                                            text_section.style.color = TYPING_COLOR;
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
                                let mut iter = q_child_with_text.iter_many_mut(child);
                                while let Some(mut text) = iter.fetch_next() {
                                    // Track if there is a mistake
                                    let mut made_a_mistake = false;
                                    if let Some(text_section) =
                                        text.sections.get_mut(currently_being_typed.index + 1)
                                    {
                                        if text_section.value == pressed_letter {
                                            // Player is continuing to type this enemy
                                            text_section.style.color = TYPING_COLOR;
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
                                            section.style.color = STANDARD_TEXT_COLOR;
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

pub fn setup_list_of_words_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let words_handle =
        WordsHandle(asset_server.load("words/thousand_most_frequent_words.words.toml"));
    commands.insert_resource(words_handle);
}

pub fn handle_text_when_enemies_collide(
    mut commands: Commands,
    mut q_parent_with_enemy: Query<
        (
            Entity,
            &Transform,
            &Children,
            &EnemySpawnPoint,
            &PathCheckpointNumber,
            &mut TextCollidingWith,
        ),
        With<Enemy>,
    >,
    mut q_child_with_text: Query<(&mut Transform, &Text), Without<Enemy>>,
) {
    let mut q_parent_combinations_iter = q_parent_with_enemy.iter_combinations_mut();
    while let Some(
        [(
            entity_first_enemy,
            transform_first_enemy,
            children_first,
            spawn_point_first,
            path_checkpoint_number_first,
            mut text_colliding_with_first,
        ), (
            entity_second_enemy,
            transform_second_enemy,
            children_second,
            spawn_point_second,
            path_checkpoint_number_second,
            mut text_colliding_with_second,
        )],
    ) = q_parent_combinations_iter.fetch_next()
    {
        let text_collision_instructions = query_texts_and_check_if_it_collides(
            &q_child_with_text,
            children_first,
            children_second,
            transform_first_enemy,
            transform_second_enemy,
            false,
        );

        // If there is a collision, then change position of text
        if let Some((text_collision_bool, text_collision_adjustments)) = text_collision_instructions
        {
            match text_collision_bool {
                // First enemy's text will be moved up
                true => {
                    // Add component to enemy indicating that currently texts are colliding
                    text_colliding_with_first.entity_colliding_with = entity_second_enemy;
                    change_position_of_text(
                        &mut q_child_with_text,
                        children_first,
                        text_collision_adjustments,
                    );
                }
                // Second enemy's text will be moved up
                false => {
                    // Add component to enemy indicating that currently texts are colliding
                    text_colliding_with_second.entity_colliding_with = entity_first_enemy;
                    change_position_of_text(
                        &mut q_child_with_text,
                        children_second,
                        text_collision_adjustments,
                    );
                }
            };
        }
    }
}

/// Queries the texts given the children entities of the enemies (the children are the text bundles)
/// and checks if the texts are colliding.
///
/// Returns an Option bool with true if the first enemy's text has to be moved and false if the second enemy's text has to be moved
/// with the needed adjustment in the Vec3. If there is no collision, None is returned.
fn query_texts_and_check_if_it_collides(
    q_child_with_text: &Query<(&mut Transform, &Text), Without<Enemy>>,
    children_first: &Children,
    children_second: &Children,
    transform_first_enemy: &Transform,
    transform_second_enemy: &Transform,
    skip_check_if_y_transforms_of_texts_are_equal: bool,
) -> Option<(bool, Vec3)> {
    let vec_of_both_texts: Vec<(&Transform, &Text)> = q_child_with_text
        .iter_many(children_first.iter().chain(children_second.iter()))
        .collect();
    let (transform_first_text, text_first_text) = vec_of_both_texts
        .get(0)
        .expect("Enemies should have text child");
    let (transform_second_text, text_second_text) = vec_of_both_texts
        .get(1)
        .expect("Enemies should have text child");

    check_if_text_is_colliding(
        text_first_text,
        text_second_text,
        &transform_first_enemy.translation,
        &transform_second_enemy.translation,
        &transform_first_text.translation,
        &transform_second_text.translation,
        skip_check_if_y_transforms_of_texts_are_equal,
    )
}

/// Checks if the texts of the first and second enemy are colliding.
///
/// Returns an Option bool with true if the first enemy's text has to be moved and false if the second enemy's text has to be moved
/// with the needed adjustment in the Vec3. If there is no collision, None is returned.
fn check_if_text_is_colliding(
    text_first_text: &Text,
    text_second_text: &Text,
    translation_first_enemy: &Vec3,
    translation_second_enemy: &Vec3,
    translation_first_text: &Vec3,
    translation_second_text: &Vec3,
    skip_check_if_y_transforms_of_texts_are_equal: bool,
) -> Option<(bool, Vec3)> {
    let font_size = text_first_text
        .sections
        .get(0)
        .expect("Text shouldn't be empty")
        .style
        .font_size;
    // Font size * 0.5 is an estimation for the size in pixels of one letter
    let first_text_pixel_size: f32 = text_first_text.sections.len() as f32 * 0.5 * font_size;
    let second_text_pixel_size: f32 = text_second_text.sections.len() as f32 * 0.5 * font_size;

    let mut is_colliding: Option<bool> = None;
    // To do add collision management for different directions enemies could be coming from
    if (translation_first_enemy.y - translation_second_enemy.y).abs() < 10.0 {
        if (translation_first_enemy.x - translation_second_enemy.x).abs()
            < 0.5 * first_text_pixel_size + 0.5 * second_text_pixel_size
        {
            if translation_first_text.y == translation_second_text.y
                || skip_check_if_y_transforms_of_texts_are_equal
            {
                // println!("There is collision!");
                // We have liftoff (collision)!
                if translation_first_enemy.x - translation_second_enemy.x <= 0.0 {
                    // First text needs to be moved
                    return Some((true, Vec3::new(0.0, enemies::TEXT_HEIGHT, 0.0)));
                } else {
                    // Second text needs to be moved
                    return Some((false, Vec3::new(0.0, enemies::TEXT_HEIGHT, 0.0)));
                };
            }
        }
    }
    None
}

fn change_position_of_text(
    q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
    children: &Children,
    text_collision_adjustments: Vec3,
) {
    let mut text_iter = q_child_with_text.iter_many_mut(children);
    if let Some((mut text_transform, _)) = text_iter.fetch_next() {
        text_transform.translation += text_collision_adjustments;
    }
}

fn reset_height_of_text(
    q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
    children: &Children,
) {
    let mut text_iter = q_child_with_text.iter_many_mut(children);
    if let Some((mut text_transform, _)) = text_iter.fetch_next() {
        text_transform.translation = Vec3::new(0.0, TEXT_HEIGHT, TEXT_Z_VALUE);
    }
}

pub fn reset_text_height_when_enemies_passed_each_other(
    mut commands: Commands,
    q_parent_with_enemy: Query<
        (Entity, &Transform, &Children, Option<&TextCollidingWith>),
        With<Enemy>,
    >,
    mut q_child_with_text: Query<(&mut Transform, &Text), Without<Enemy>>,
) {
    let mut q_parent_combinations_iter = q_parent_with_enemy.iter_combinations();
    while let Some(
        [(
            entity_first_enemy,
            transform_first_enemy,
            children_first,
            text_colliding_option_first_enemy,
        ), (
            entity_second_enemy,
            transform_second_enemy,
            children_second,
            text_colliding_option_second_enemy,
        )],
    ) = q_parent_combinations_iter.fetch_next()
    {
        if let Some(first_enemy_text_colliding_with) = text_colliding_option_first_enemy {
            if first_enemy_text_colliding_with.entity_colliding_with == entity_second_enemy {
                handle_resetting_text(
                    children_first,
                    transform_first_enemy,
                    children_second,
                    transform_second_enemy,
                    &mut q_child_with_text,
                );
            }
        } else if let Some(second_enemy_text_colliding_with) = text_colliding_option_second_enemy {
            if second_enemy_text_colliding_with.entity_colliding_with == entity_first_enemy {
                handle_resetting_text(
                    children_second,
                    transform_second_enemy,
                    children_first,
                    transform_first_enemy,
                    &mut q_child_with_text,
                )
            }
        };
    }
}

fn handle_resetting_text(
    colliding_enemy_child: &Children,
    colliding_enemy_transform: &Transform,
    non_colliding_enemy_child: &Children,
    non_colliding_enemy_transform: &Transform,
    mut q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
) {
    if let None = query_texts_and_check_if_it_collides(
        &q_child_with_text,
        colliding_enemy_child,
        non_colliding_enemy_child,
        colliding_enemy_transform,
        non_colliding_enemy_transform,
        true,
    ) {
        reset_height_of_text(&mut q_child_with_text, colliding_enemy_child)
    };
}

pub fn reset_text_height_when_colliding_enemy_is_removed(
    mut removed: RemovedComponents<Enemy>,
    enemies_with_colliding_text_query: Query<
        (Entity, &Transform, &Children, &TextCollidingWith),
        With<Enemy>,
    >,
    mut q_child_with_text: Query<(&mut Transform, &Text), Without<Enemy>>,
) {
    for removed_enemy in removed.read() {
        for (_, _, enemy_children, enemy_text_colliding_with) in
            enemies_with_colliding_text_query.iter()
        {
            if removed_enemy == enemy_text_colliding_with.entity_colliding_with {
                reset_height_of_text(&mut q_child_with_text, enemy_children);
            }
        }
    }
}
