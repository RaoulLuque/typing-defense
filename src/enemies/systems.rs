use super::*;

use bevy::hierarchy::Children;

pub fn spawn_enemy(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::default(),
                texture: asset_server.load("sprites/skull.png"),
                ..default()
            },
            Enemy {},
        ))
        .with_children(|parent| {
            parent.spawn(Text2dBundle {
                text: Text {
                    sections: turn_string_literal_into_vec_of_text_sections("lol"),
                    alignment: TextAlignment::Center,
                    linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                },
                // ensure the text is drawn on top of the box
                transform: Transform::from_xyz(0.0, 50.0, 0.0),
                ..default()
            });
        });
}

pub fn update_text_from_enemies_on_button_press(
    mut commands: Commands,
    mut is_something_being_typed: ResMut<IsSomethingBeingTyped>,
    keyboard_input: Res<Input<KeyCode>>,
    mut q_parent: Query<(Entity, Option<&mut CurrentlyBeingTyped>, &Children), With<Enemy>>,
    mut q_child: Query<&mut Text>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Iterate over all enemies with children and get typing index if necessary
        for (entity_id, currently_being_typed, child) in q_parent.iter_mut() {
            if !is_something_being_typed.indicator {
                // If nothing is currently being typed
                if let Some(_) = currently_being_typed {
                    // This should never happen but
                    commands.entity(entity_id).remove::<CurrentlyBeingTyped>();
                }
                let mut iter = q_child.iter_many_mut(child);
                while let Some(mut text) = iter.fetch_next() {
                    if let Some(text_section) = text.sections.get_mut(0) {
                        text_section.style.color = Color::ORANGE_RED;
                    }
                }

                // Insert the currently being typed component into enemy
                commands
                    .entity(entity_id)
                    .insert(CurrentlyBeingTyped { index: 0 });
                // Set global resource that something is being typed accordingly
                is_something_being_typed.indicator = true;
            } else {
                // Something is being typed already
                if let Some(mut currently_being_typed) = currently_being_typed {
                    let mut iter = q_child.iter_many_mut(child);
                    while let Some(mut text) = iter.fetch_next() {
                        if let Some(text_section) =
                            text.sections.get_mut(currently_being_typed.index + 1)
                        {
                            text_section.style.color = Color::ORANGE_RED;
                        }
                        currently_being_typed.index = currently_being_typed.index + 1;
                    }
                }
            }
        }
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
