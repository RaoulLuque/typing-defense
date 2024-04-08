use bevy::input::{keyboard::KeyboardInput, ButtonState};

use enemies::rounds_and_indicators::resources::StreakIndicator;

use self::enemies::movement::{
    components::{EnemySpawnPoint, PathCheckpointNumber},
    systems::TurnInstruction,
};

use super::*;

/// Event that used whenever an enemy was typed
#[derive(Event)]
pub struct EnemyTypedEvent();

/// System that updates which enemies are being typed
pub fn update_text_from_enemies_on_button_press(
    mut commands: Commands,
    mut enemies_being_typed: ResMut<EnemiesBeingTyped>,
    mut streak_indicator: ResMut<StreakIndicator>,
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut q_parent_with_enemy: Query<
        (Entity, Option<&mut CurrentlyBeingTyped>, &Children),
        With<Enemy>,
    >,
    mut q_child_with_text: Query<&mut Text>,
    mut enemy_typed_event: EventWriter<EnemyTypedEvent>,
) {
    for key_event in keyboard_input_events.read() {
        if key_event.state != ButtonState::Pressed {
            continue;
        } else {
            // Case where key is being pressed
            let pressed_key = key_event.key_code;
            // Check if esc or backspace was just pressed and reset all enemies if so
            if pressed_key == KeyCode::Backspace {
                for (entity_id, currently_being_typed, child) in q_parent_with_enemy.iter_mut() {
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
                for (entity_id, currently_being_typed, child) in q_parent_with_enemy.iter_mut() {
                    if !enemies_being_typed.indicator && !made_a_mistake_global {
                        // If nothing is currently being typed
                        let mut iter = q_child_with_text.iter_many_mut(child);
                        while let Some(mut text) = iter.fetch_next() {
                            let number_of_letter_in_word = text.sections.len();
                            if let Some(text_section) = text.sections.get_mut(0) {
                                if text_section.value == pressed_letter {
                                    streak_indicator.number += 1;
                                    if number_of_letter_in_word == 1 {
                                        // You got "typed"
                                        // Enemy only consists of one letter - You got "typed"
                                        // Despawn entity and remove entity from list of enemies that are currently being typed
                                        commands.entity(entity_id).despawn_recursive();
                                        enemy_typed_event.send(EnemyTypedEvent {});
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
                                        streak_indicator.number += 1;
                                        if currently_being_typed.index == text.sections.len() - 1 {
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
                                            enemy_typed_event.send(EnemyTypedEvent {});
                                        }
                                    } else {
                                        // Player is typing another enemy or has made a mistake
                                        made_a_mistake = true;
                                        made_a_mistake_global = true;
                                    }
                                }
                                if made_a_mistake {
                                    // Reset text of current enemy
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
                                    if made_a_mistake_global {
                                        streak_indicator.number = 0;
                                    }
                                }
                            }
                        }
                    }
                }
                // Case where there were no enemies being typed before but now there is one
                // This is done outside of the for loop in order not to exclude partial matches
                if !enemies_being_typed.indicator && enemies_being_typed.vec_of_enemies.len() > 0 {
                    // Set global resource that something is being typed accordingly
                    enemies_being_typed.indicator = true;
                }
            }
        }
    }
}

/// Maps keys to letters and returns none if the key is not needed
fn key_to_letter(key: KeyCode) -> Option<String> {
    match key {
        KeyCode::KeyA => Some("a".to_string()),
        KeyCode::KeyB => Some("b".to_string()),
        KeyCode::KeyC => Some("c".to_string()),
        KeyCode::KeyD => Some("d".to_string()),
        KeyCode::KeyE => Some("e".to_string()),
        KeyCode::KeyF => Some("f".to_string()),
        KeyCode::KeyG => Some("g".to_string()),
        KeyCode::KeyH => Some("h".to_string()),
        KeyCode::KeyI => Some("i".to_string()),
        KeyCode::KeyJ => Some("j".to_string()),
        KeyCode::KeyK => Some("k".to_string()),
        KeyCode::KeyL => Some("l".to_string()),
        KeyCode::KeyM => Some("m".to_string()),
        KeyCode::KeyN => Some("n".to_string()),
        KeyCode::KeyO => Some("o".to_string()),
        KeyCode::KeyP => Some("p".to_string()),
        KeyCode::KeyQ => Some("q".to_string()),
        KeyCode::KeyR => Some("r".to_string()),
        KeyCode::KeyS => Some("s".to_string()),
        KeyCode::KeyT => Some("t".to_string()),
        KeyCode::KeyU => Some("u".to_string()),
        KeyCode::KeyV => Some("v".to_string()),
        KeyCode::KeyW => Some("w".to_string()),
        KeyCode::KeyX => Some("x".to_string()),
        KeyCode::KeyY => Some("z".to_string()),
        KeyCode::KeyZ => Some("y".to_string()),
        KeyCode::Backslash => Some("'".to_string()),
        _ => None,
    }
}

/// Sets up the asset list of words used for typing
pub fn setup_list_of_words_asset(mut commands: Commands, asset_server: Res<AssetServer>) {
    let words_handle =
        WordsHandle(asset_server.load("words/thousand_most_frequent_words.words.toml"));
    commands.insert_resource(words_handle);
}

/// System that handles the movement of text when enemies collide.
/// Moves the text of the approaching enemy upward
pub fn handle_text_when_enemies_collide(
    mut q_parent_with_enemy: Query<
        (
            Entity,
            &Transform,
            &Children,
            &EnemySpawnPoint,
            &PathCheckpointNumber,
            &mut CollidingWith,
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
            enemy_spawn_point_first_enemy,
            path_checkpoint_number_first_enemy,
            mut text_colliding_with_first,
        ), (
            entity_second_enemy,
            transform_second_enemy,
            children_second,
            enemy_spawn_point_second_enemy,
            path_checkpoint_number_second_enemy,
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
            enemy_spawn_point_first_enemy,
            enemy_spawn_point_second_enemy,
            path_checkpoint_number_first_enemy,
            path_checkpoint_number_second_enemy,
            false,
        );

        if !(text_colliding_with_first.entity_colliding_with == entity_second_enemy
            || text_colliding_with_second.entity_colliding_with == entity_first_enemy)
        {
            // If there is a collision, then change position of text
            if let Some((text_collision_bool, colliding_text_new_translation)) =
                text_collision_instructions
            {
                match text_collision_bool {
                    // First enemy's text will be moved up
                    true => {
                        // Add component to enemy indicating that currently texts are colliding
                        text_colliding_with_first.entity_colliding_with = entity_second_enemy;
                        change_position_of_text(
                            &mut q_child_with_text,
                            children_first,
                            colliding_text_new_translation,
                        );
                    }
                    // Second enemy's text will be moved up
                    false => {
                        // Add component to enemy indicating that currently texts are colliding
                        text_colliding_with_second.entity_colliding_with = entity_first_enemy;
                        change_position_of_text(
                            &mut q_child_with_text,
                            children_second,
                            colliding_text_new_translation,
                        );
                    }
                };
            }
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
    enemy_spawn_point_first_enemy: &EnemySpawnPoint,
    enemy_spawn_point_second_enemy: &EnemySpawnPoint,
    path_checkpoint_number_first_enemy: &PathCheckpointNumber,
    path_checkpoint_number_second_enemy: &PathCheckpointNumber,
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
        enemy_spawn_point_first_enemy,
        enemy_spawn_point_second_enemy,
        path_checkpoint_number_first_enemy,
        path_checkpoint_number_second_enemy,
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
    enemy_spawn_point_first_enemy: &EnemySpawnPoint,
    enemy_spawn_point_second_enemy: &EnemySpawnPoint,
    path_checkpoint_number_first_enemy: &PathCheckpointNumber,
    path_checkpoint_number_second_enemy: &PathCheckpointNumber,
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

    let turn_instruction_first_enemy = super::movement::systems::get_current_turn_instruction(
        enemy_spawn_point_first_enemy,
        path_checkpoint_number_first_enemy.number,
    );
    let turn_instruction_second_enemy = super::movement::systems::get_current_turn_instruction(
        enemy_spawn_point_second_enemy,
        path_checkpoint_number_second_enemy.number,
    );

    if turn_instruction_first_enemy == turn_instruction_second_enemy {
        match turn_instruction_first_enemy {
            TurnInstruction::Left | TurnInstruction::Right => {
                if (translation_first_enemy.y - translation_second_enemy.y).abs() < 10.0 {
                    if (translation_first_enemy.x - translation_second_enemy.x).abs()
                        < 0.5 * first_text_pixel_size + 0.5 * second_text_pixel_size
                    {
                        if translation_first_text.y == translation_second_text.y
                            || skip_check_if_y_transforms_of_texts_are_equal
                        {
                            // We have liftoff (collision)!
                            if translation_first_enemy.x <= translation_second_enemy.x {
                                match turn_instruction_first_enemy {
                                    // Second text needs to be moved
                                    TurnInstruction::Left => {
                                        return Some((
                                            false,
                                            Vec3::new(
                                                translation_first_text.x,
                                                translation_first_text.y + enemies::TEXT_HEIGHT,
                                                translation_first_text.z,
                                            ),
                                        ))
                                    }
                                    // First text needs to be moved
                                    TurnInstruction::Right => {
                                        return Some((
                                            true,
                                            Vec3::new(
                                                translation_second_text.x,
                                                translation_second_text.y + enemies::TEXT_HEIGHT,
                                                translation_second_text.z,
                                            ),
                                        ))
                                    }
                                    // This can never happen
                                    _ => return None,
                                }
                            } else {
                                match turn_instruction_first_enemy {
                                    // Second text needs to be moved
                                    TurnInstruction::Right => {
                                        return Some((
                                            false,
                                            Vec3::new(
                                                translation_first_text.x,
                                                translation_first_text.y + enemies::TEXT_HEIGHT,
                                                translation_first_text.z,
                                            ),
                                        ))
                                    }
                                    // First text needs to be moved
                                    TurnInstruction::Left => {
                                        return Some((
                                            true,
                                            Vec3::new(
                                                translation_second_text.x,
                                                translation_second_text.y + enemies::TEXT_HEIGHT,
                                                translation_second_text.z,
                                            ),
                                        ))
                                    }
                                    // This can never happen
                                    _ => return None,
                                }
                            };
                        }
                    }
                }
            }
            TurnInstruction::Up | TurnInstruction::Down => {
                if (translation_first_enemy.x - translation_second_enemy.x).abs() < 10.0 {
                    if (translation_first_enemy.y - translation_second_enemy.y).abs() < TEXT_HEIGHT
                    {
                        if translation_first_text.y == translation_second_text.y
                            || skip_check_if_y_transforms_of_texts_are_equal
                        {
                            // We have liftoff (collision)!
                            if translation_first_enemy.y <= translation_second_enemy.y {
                                match turn_instruction_first_enemy {
                                    // Second text needs to be moved
                                    TurnInstruction::Up => {
                                        return Some((
                                            false,
                                            Vec3::new(
                                                translation_first_text.x,
                                                translation_first_text.y + enemies::TEXT_HEIGHT,
                                                translation_first_text.z,
                                            ),
                                        ))
                                    }
                                    // Second text needs to be moved
                                    TurnInstruction::Down => {
                                        return Some((
                                            false,
                                            Vec3::new(
                                                translation_first_text.x,
                                                translation_first_text.y + enemies::TEXT_HEIGHT,
                                                translation_first_text.z,
                                            ),
                                        ))
                                    }
                                    // This can never happen
                                    _ => return None,
                                }
                            } else {
                                match turn_instruction_first_enemy {
                                    // First text needs to be moved
                                    TurnInstruction::Up => {
                                        return Some((
                                            true,
                                            Vec3::new(
                                                translation_second_text.x,
                                                translation_second_text.y + enemies::TEXT_HEIGHT,
                                                translation_second_text.z,
                                            ),
                                        ))
                                    }
                                    // First text needs to be moved
                                    TurnInstruction::Down => {
                                        return Some((
                                            true,
                                            Vec3::new(
                                                translation_second_text.x,
                                                translation_second_text.y + enemies::TEXT_HEIGHT,
                                                translation_second_text.z,
                                            ),
                                        ))
                                    }
                                    // This can never happen
                                    _ => return None,
                                }
                            };
                        }
                    }
                }
            }
        }
    }
    None
}

/// Changes the position of text according to the given new translation
fn change_position_of_text(
    q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
    children: &Children,
    colliding_text_new_translation: Vec3,
) {
    let mut text_iter = q_child_with_text.iter_many_mut(children);
    if let Some((mut text_transform, _)) = text_iter.fetch_next() {
        text_transform.translation = colliding_text_new_translation;
    }
}

/// Resets the height of text
fn reset_height_of_text(
    q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
    children: &Children,
    colliding_text_colliding_with: &mut CollidingWith,
) {
    let mut text_iter = q_child_with_text.iter_many_mut(children);
    if let Some((mut text_transform, _)) = text_iter.fetch_next() {
        text_transform.translation = Vec3::new(0.0, TEXT_HEIGHT, TEXT_Z_VALUE);
        colliding_text_colliding_with.entity_colliding_with = Entity::PLACEHOLDER;
    }
}

/// Resets text height when enemies passed each other
pub fn reset_text_height_when_enemies_passed_each_other(
    mut q_parent_with_enemy: Query<
        (
            Entity,
            &Transform,
            &Children,
            &EnemySpawnPoint,
            &PathCheckpointNumber,
            &mut CollidingWith,
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
            enemy_spawn_point_first_enemy,
            path_checkpoint_number_first_enemy,
            mut text_colliding_with_first_enemy,
        ), (
            entity_second_enemy,
            transform_second_enemy,
            children_second,
            enemy_spawn_point_second_enemy,
            path_checkpoint_number_second_enemy,
            mut text_colliding_with_second_enemy,
        )],
    ) = q_parent_combinations_iter.fetch_next()
    {
        if text_colliding_with_first_enemy.entity_colliding_with == entity_second_enemy {
            handle_checking_collision_and_resetting_text(
                children_first,
                transform_first_enemy,
                children_second,
                transform_second_enemy,
                enemy_spawn_point_first_enemy,
                enemy_spawn_point_second_enemy,
                path_checkpoint_number_first_enemy,
                path_checkpoint_number_second_enemy,
                &mut q_child_with_text,
                &mut text_colliding_with_first_enemy,
            );
        } else if text_colliding_with_second_enemy.entity_colliding_with == entity_first_enemy {
            handle_checking_collision_and_resetting_text(
                children_second,
                transform_second_enemy,
                children_first,
                transform_first_enemy,
                enemy_spawn_point_first_enemy,
                enemy_spawn_point_second_enemy,
                path_checkpoint_number_first_enemy,
                path_checkpoint_number_second_enemy,
                &mut q_child_with_text,
                &mut text_colliding_with_second_enemy,
            )
        };
    }
}

/// Handles checking collisions and resetting text
fn handle_checking_collision_and_resetting_text(
    colliding_enemy_child: &Children,
    colliding_enemy_transform: &Transform,
    non_colliding_enemy_child: &Children,
    non_colliding_enemy_transform: &Transform,
    enemy_spawn_point_first_enemy: &EnemySpawnPoint,
    enemy_spawn_point_second_enemy: &EnemySpawnPoint,
    path_checkpoint_number_first_enemy: &PathCheckpointNumber,
    path_checkpoint_number_second_enemy: &PathCheckpointNumber,
    mut q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
    colliding_text_colliding_with: &mut CollidingWith,
) {
    if let None = query_texts_and_check_if_it_collides(
        &q_child_with_text,
        colliding_enemy_child,
        non_colliding_enemy_child,
        colliding_enemy_transform,
        non_colliding_enemy_transform,
        enemy_spawn_point_first_enemy,
        enemy_spawn_point_second_enemy,
        path_checkpoint_number_first_enemy,
        path_checkpoint_number_second_enemy,
        true,
    ) {
        reset_height_of_text(
            &mut q_child_with_text,
            colliding_enemy_child,
            colliding_text_colliding_with,
        )
    };
}

/// Lowers text stepwise when colliding enemy is removed instead of resetting text height
pub fn lower_text_stepwise_when_colliding_enemy_is_removed(
    mut commands: Commands,
    mut enemies_with_colliding_text_query: Query<
        (Entity, &Transform, &Children, &mut CollidingWith),
        With<Enemy>,
    >,
    mut q_child_with_text: Query<(&mut Transform, &Text), Without<Enemy>>,
) {
    for (_, _, enemy_children, mut enemy_text_colliding_with) in
        enemies_with_colliding_text_query.iter_mut()
    {
        if let None = commands.get_entity(enemy_text_colliding_with.entity_colliding_with) {
            lower_text_stepwise(
                &mut q_child_with_text,
                enemy_children,
                &mut enemy_text_colliding_with,
            );
        }
    }
}

/// Lower text stepwise
fn lower_text_stepwise(
    q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
    children: &Children,
    colliding_text_colliding_with: &mut CollidingWith,
) {
    let mut text_iter = q_child_with_text.iter_many_mut(children);
    if let Some((mut text_transform, _)) = text_iter.fetch_next() {
        if text_transform.translation.y > TEXT_HEIGHT {
            text_transform.translation = Vec3::new(
                0.0,
                text_transform.translation.y - TEXT_HEIGHT,
                TEXT_Z_VALUE,
            );
            if text_transform.translation.y == TEXT_HEIGHT {
                colliding_text_colliding_with.entity_colliding_with = Entity::PLACEHOLDER;
            }
        }
    }
}

/// Check if the text of the colliding enemy has moved
pub fn check_if_colliding_text_has_moved(
    q_parent_with_enemy: Query<(Entity, &Children, &CollidingWith), With<Enemy>>,
    mut q_child_with_text: Query<(&mut Transform, &Text), Without<Enemy>>,
) {
    let mut q_parent_combinations_iter = q_parent_with_enemy.iter_combinations();
    while let Some(
        [(entity_first_enemy, children_first, text_colliding_with_first_enemy), (entity_second_enemy, children_second, text_colliding_with_second_enemy)],
    ) = q_parent_combinations_iter.fetch_next()
    {
        if text_colliding_with_first_enemy.entity_colliding_with == entity_second_enemy {
            check_if_colliding_text_is_right_above_and_change_position_of_text_if_necessary(
                &mut q_child_with_text,
                children_first,
                children_second,
            );
        } else if text_colliding_with_second_enemy.entity_colliding_with == entity_first_enemy {
            check_if_colliding_text_is_right_above_and_change_position_of_text_if_necessary(
                &mut q_child_with_text,
                children_second,
                children_first,
            );
        };
    }
}

/// Checks if the text of the colliding enemy has moved and adjusts position accordingly if necessary
fn check_if_colliding_text_is_right_above_and_change_position_of_text_if_necessary(
    q_child_with_text: &mut Query<(&mut Transform, &Text), Without<Enemy>>,
    child_text_that_is_moved_above: &Children,
    child_text_of_enemy_that_is_collided_with: &Children,
) {
    let mut text_iter_collided_with =
        q_child_with_text.iter_many(child_text_of_enemy_that_is_collided_with);
    let mut y_translation_of_text_that_is_collided_with_option: Option<f32> = None;
    if let Some((text_transform, _)) = text_iter_collided_with.fetch_next() {
        y_translation_of_text_that_is_collided_with_option = Some(text_transform.translation.y);
    }

    let mut text_iter = q_child_with_text.iter_many_mut(child_text_that_is_moved_above);
    if let Some((mut text_transform, _)) = text_iter.fetch_next() {
        if let Some(y_translation_of_text_that_is_collided_with) =
            y_translation_of_text_that_is_collided_with_option
        {
            if text_transform.translation.y
                != y_translation_of_text_that_is_collided_with + TEXT_HEIGHT
            {
                text_transform.translation.y =
                    y_translation_of_text_that_is_collided_with + TEXT_HEIGHT;
            }
        }
    }
}
