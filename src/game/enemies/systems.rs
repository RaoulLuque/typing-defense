use self::movement::components::{EnemySpawnPoint, PathCheckpointNumber};
use self::text::components::CollidingWith;

use super::*;

use crate::game::rounds::resources::*;

use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;
use rand::Rng;

// Chance of spawning an enemy every super::resources::ENEMY_SPAWN_TIME seconds
pub const CHANCE_OF_SPAWNING_ENEMY: f64 = 1.0;
// Base value which is divided by the enemy speed to get the animation speed - lower = faster
pub const BASE_ANIMATION_SPEED: f32 = 3.0;
// Scale factor by which enemy sprites are scaled - higher = bigger
pub const ENEMY_SPRITE_SCALE_FACTOR: f32 = 1.4;

// Standard text color
pub const STANDARD_TEXT_COLOR: Color = Color::AZURE;
// Text color while typing
pub const TYPING_COLOR: Color = Color::ORANGE_RED;
// Font size for text
pub const TEXT_FONT_SIZE: f32 = 60.0;
// Standard enemy text height (height in pixels that the text is above enemies)
pub const TEXT_HEIGHT: f32 = 50.0;
// Standard text z value (in order to be in front of decorations)
pub const TEXT_Z_VALUE: f32 = 1.0;

#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Words {
    pub vec_of_words: Vec<String>,
}

#[derive(Bundle)]
struct EnemyBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    entity_type: Enemy,
    spawn_point: EnemySpawnPoint,
    speed: Speed,
    walking_animation: WalkingAnimation,
    enemy_type: EnemyType,
    path_checkpoint_number: PathCheckpointNumber,
    text_collision: CollidingWith,
    name: Name,
}

pub fn randomly_spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut last_enemy_spawn_point: ResMut<LastEnemySpawnPoint>,
    mut number_of_enemies_spawned_this_round: ResMut<NumberOfEnemiesSpawnedCurrentRound>,
    max_number_of_enemies_this_round: Res<MaxNumberOfEnemiesCurrentRound>,
    number_of_enemies_typed_current_round: Res<NumberOfEnemiesTypedCurrentRound>,
    enemy_base_speed_this_round: Res<EnemyBaseSpeedCurrentRound>,
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    words_handle: Res<WordsHandle>,
    words: Res<Assets<Words>>,
) {
    // Spawn only as many enemies as is planned for this round
    if number_of_enemies_spawned_this_round.number < max_number_of_enemies_this_round.number {
        // Get thread rng once for better performance
        let mut rng = rand::thread_rng();
        if (enemy_spawn_timer.timer.finished() && rng.gen_bool(CHANCE_OF_SPAWNING_ENEMY))
            || (number_of_enemies_spawned_this_round.number
                == number_of_enemies_typed_current_round.number)
        {
            if number_of_enemies_spawned_this_round.number
                == number_of_enemies_typed_current_round.number
            {
                enemy_spawn_timer
                    .timer
                    .set_elapsed(std::time::Duration::from_secs_f32(0.0));
            }
            let window = window_query.get_single().unwrap();

            // Get a random spawn point
            let spawn_point = last_enemy_spawn_point
                .spawn_point
                .next_spawn_point_excluding_self(&mut rng);
            last_enemy_spawn_point.spawn_point = spawn_point;
            let spawn_point_transform =
                movement::systems::generate_spawn_point_transform_from_enum(spawn_point, window);

            // Get random enemy sprite
            let enemy_type: EnemyType = rng.gen();
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
            let flip_on_y_axis =
                movement::systems::check_if_sprite_needs_to_be_flipped_from_spawnpoint(spawn_point);

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
                    .spawn(EnemyBundle {
                        sprite_sheet_bundle: SpriteSheetBundle {
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
                        entity_type: Enemy {},
                        spawn_point,
                        speed: Speed { speed: speed },
                        walking_animation,
                        enemy_type,
                        path_checkpoint_number: PathCheckpointNumber::default(),
                        text_collision: CollidingWith::default(),
                        name: Name::new(word_for_enemy.clone()),
                    })
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
                            transform: Transform::from_xyz(0.0, TEXT_HEIGHT, TEXT_Z_VALUE),
                            ..default()
                        });
                    });
            }
            number_of_enemies_spawned_this_round.number += 1;
        }
    }
}

/// Returns the necessary info in order to generate a spritesheet for each enemy type
pub fn generate_sprite_information_from_enemy_type(
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

// Turns a string literal into a vector of text sections each containing one character from the string literal
fn turn_string_literal_into_vec_of_text_sections(string_literal: &str) -> Vec<TextSection> {
    string_literal
        .chars()
        .map(|x| {
            TextSection::new(
                x.to_string(),
                TextStyle {
                    font_size: TEXT_FONT_SIZE,
                    color: STANDARD_TEXT_COLOR,
                    ..default()
                },
            )
        })
        .collect()
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
