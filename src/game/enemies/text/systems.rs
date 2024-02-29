use bevy::input::{keyboard::KeyboardInput, ButtonState};

use enemies::rounds::resources::NumberOfEnemiesTypedCurrentRound;

use super::*;

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
                                let mut iter = q_child.iter_many_mut(child);
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
