use bevy::window::PrimaryWindow;
use rand::seq::SliceRandom;

use crate::menu::systems::Restart;

use super::enemies::components::{Enemy, Speed, WalkingAnimation};
use super::enemies::movement::components::{EnemySpawnPoint, PathCheckpointNumber};
use super::enemies::resources::WordsHandle;
use super::enemies::systems::Words;
use super::enemies::text::components::CollidingWith;
use super::rounds_and_indicators::resources::{EnemyBaseSpeedCurrentRound, RoundNumber};
use super::*;

/// The number of words per boss equals, the round number * this number
pub const BOSS_WORD_COUNT_MULTIPLIER: u32 = 3;

/// Spawns the boss (king slime)
pub fn spawn_boss(
    mut commands: Commands,
    round_number: Res<RoundNumber>,
    enemy_base_speed_this_round: Res<EnemyBaseSpeedCurrentRound>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    words_handle: Res<WordsHandle>,
    words: Res<Assets<Words>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Spawn boss if round number is multiple of 10
    if round_number.number % 10 == 0 {
        // Get thread rng once for better performance
        let mut rng = rand::thread_rng();

        // Get spawn point for all "ghost" enemies and the boss
        let window = window_query.get_single().expect("Window should exist");
        let spawn_point = EnemySpawnPoint::BottomLeft;
        let spawn_point_transform =
            super::enemies::movement::systems::generate_spawn_point_transform_from_enum(
                spawn_point,
                window,
            );

        // Get slime texture handle
        let texture_handle: Handle<Image> = asset_server.load(format!("sprites/enemies/slime.png"));
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(44.0, 30.0), 10, 1, None, None);
        let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

        // Setup slime walking animation
        let walking_animation: WalkingAnimation = WalkingAnimation {
            length_of_animation: 10,
            animation_timer: Timer::from_seconds(
                super::enemies::systems::BASE_ANIMATION_SPEED / enemy_base_speed_this_round.speed,
                TimerMode::Repeating,
            ),
        };

        // Spawn boss (slime)
        commands.spawn((
            SpriteSheetBundle {
                transform: spawn_point_transform,
                sprite: TextureAtlasSprite {
                    flip_x: true,
                    index: 0,
                    custom_size: Some(Vec2::new(
                        super::enemies::systems::ENEMY_SPRITE_SCALE_FACTOR * 30.0,
                        super::enemies::systems::ENEMY_SPRITE_SCALE_FACTOR * 30.0,
                    )),
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            walking_animation,
            spawn_point,
            Speed {
                speed: enemy_base_speed_this_round.speed,
            },
            PathCheckpointNumber::default(),
            Name::new("King Slime".to_string()),
            Boss {},
        ));

        // BOSS_WORD_COUNT_MULTIPLIER * round number "ghost" enemies (without sprites) at the same
        // spot to create the illusion of one enemy with a lot of words.
        for _ in 0..BOSS_WORD_COUNT_MULTIPLIER * round_number.number {
            if let Some(word) = words.get(words_handle.0.id()) {
                // Get random word from list
                let word_for_enemy = word
                    .vec_of_words
                    .choose(&mut rng)
                    .expect("The list of words shouldn't be empty");

                // Get ghost texture handle
                let texture_handle: Handle<Image> =
                    asset_server.load(format!("sprites/enemies/ghost.png"));
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(30.0, 30.0),
                    1,
                    1,
                    None,
                    None,
                );
                let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

                commands
                    .spawn((
                        SpriteSheetBundle {
                            transform: spawn_point_transform,
                            sprite: TextureAtlasSprite {
                                flip_x: false,
                                index: 0,
                                custom_size: Some(Vec2::new(super::enemies::systems::ENEMY_SPRITE_SCALE_FACTOR * 30.0, super::enemies::systems::ENEMY_SPRITE_SCALE_FACTOR * 30.0)),
                                ..default()
                            },
                            texture_atlas: texture_atlas_handle,
                            ..default()
                        },
                        Enemy {},
                        spawn_point,
                        Speed {
                            speed: enemy_base_speed_this_round.speed,
                        },
                        PathCheckpointNumber::default(),
                        CollidingWith::default(),
                        Name::new(word_for_enemy.clone()),
                    ))
                    .with_children(|parent| {
                        parent.spawn(Text2dBundle {
                            text: Text {
                                sections: super::enemies::systems::turn_string_literal_into_vec_of_text_sections(
                                    word_for_enemy,
                                    super::enemies::systems::STANDARD_TEXT_COLOR,
                                ),
                                alignment: TextAlignment::Center,
                                linebreak_behavior: bevy::text::BreakLineOn::NoWrap,
                            },
                            // ensure the text is drawn on top of the box
                            transform: Transform::from_xyz(0.0, super::enemies::systems::TEXT_HEIGHT, super::enemies::systems::TEXT_Z_VALUE),
                            ..default()
                        });
                    });
            }
        }
    }
}

pub fn despawn_boss(mut commands: Commands, boss_query: Query<Entity, With<Boss>>) {
    if let Ok(boss_entity) = boss_query.get_single() {
        commands.entity(boss_entity).despawn_recursive();
    }
}

pub fn despawn_boss_on_restart(
    mut commands: Commands,
    boss_query: Query<Entity, With<Boss>>,
    mut restart_event_reader: EventReader<Restart>,
) {
    for _ in restart_event_reader.read() {
        if let Ok(boss_entity) = boss_query.get_single() {
            commands.entity(boss_entity).despawn_recursive();
        }
    }
}
