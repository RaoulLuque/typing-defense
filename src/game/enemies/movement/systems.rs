use effects::components::{Explosion, ExplosionAnimation};
use enemies::rounds_and_indicators::resources::{NumberOfEnemiesUnlivedThisRound, StreakIndicator};

use crate::menu::systems::Restart;

use super::*;

/// The movement is calculated relatively to the screen height/width in order to work for multiple resolutions
/// One grid block on the background corresponds to:
/// In width: 0.03333
/// In height: 0.05263
#[derive(PartialEq, Eq)]
pub enum TurnInstruction {
    Left,
    Right,
    Up,
    Down,
}

use bevy::window::PrimaryWindow;
use TurnInstruction::*;

/// Window width in dev
const WINDOW_WIDTH_IN_DEV: f32 = 1856.0;
/// Window height in dev
const WINDOW_HEIGHT_IN_DEV: f32 = 1018.0;

/// Checkpoints for the route of the enemies coming from the top left of the screen in ratio of screen width / height coordinates
const TOP_LEFT_ROUTE_CHECKPOINTS: [(f32, f32); 6] = [
    (-0.31, 0.5),
    (-0.31, 0.31578),
    (0.0, 0.31778),
    (0.0, -0.31178),
    (0.20899, -0.31178),
    (0.20899, -1.0),
];
/// Turn instructions of the enemies coming from the top left of the screen
const TOP_LEFT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 6] =
    [Down, Right, Down, Right, Down, Down];

/// Checkpoints for the route of the enemies coming from the bottom right of the screen in ratio of screen width / height coordinates
const BOTTOM_RIGHT_ROUTE_CHECKPOINTS: [(f32, f32); 6] = [
    (0.20899, -0.5),
    (0.20899, -0.31178),
    (0.0, -0.31178),
    (0.0, 0.31778),
    (-0.31, 0.31578),
    (-0.31, 1.0),
];
/// Turn instructions of the enemies coming from the bottom right of the screen
const BOTTOM_RIGHT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 6] = [Up, Left, Up, Left, Up, Up];

/// Checkpoints for the route of the enemies coming from the top right of the screen in ratio of screen width / height coordinates
const TOP_RIGHT_ROUTE_CHECKPOINTS: [(f32, f32); 9] = [
    (0.27664, 0.5),
    (0.27664, 0.20252),
    (0.10299, 0.20252),
    (0.10299, 0.31778),
    (0.0, 0.31778),
    (0.0, -0.18552),
    (-0.20698, -0.18552),
    (-0.20698, -0.36841),
    (-1.0, -0.36841),
];
/// Turn instructions of the enemies coming from the top right of the screen
const TOP_RIGHT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 9] =
    [Down, Left, Up, Left, Down, Left, Down, Left, Left];

/// Checkpoints for the route of the enemies coming from the bottom left of the screen in ratio of screen width / height coordinates
const BOTTOM_LEFT_ROUTE_CHECKPOINTS: [(f32, f32); 9] = [
    (-0.5, -0.36841),
    (-0.20698, -0.36841),
    (-0.20698, -0.18552),
    (0.0, -0.18552),
    (0.0, 0.31778),
    (0.10299, 0.31778),
    (0.10299, 0.20252),
    (0.27664, 0.20252),
    (0.27664, 1.0),
];
/// Turn instructions of the enemies coming from the bottom left of the screen
const BOTTOM_LEFT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 9] =
    [Right, Up, Right, Up, Right, Down, Right, Up, Up];

/// Checkpoints for the route of the enemies coming from the left of the screen in ratio of screen width / height coordinates
const LEFT_ROUTE_CHECKPOINTS: [(f32, f32); 2] = [(-0.5, 0.01), (1.0, 0.01)];
/// Turn instructions of the enemies coming from the left of the screen
const LEFT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 2] = [Right, Right];

/// Checkpoints for the route of the enemies coming from the right of the screen in ratio of screen width / height coordinates
const RIGHT_ROUTE_CHECKPOINTS: [(f32, f32); 2] = [(0.5, 0.01), (-1.0, 0.01)];
/// Turn instructions of the enemies coming from the right of the screen
const RIGHT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 2] = [Left, Left];

/// System for the movement of enemies and bosses based on the checkpoints and turn instructions
pub fn update_position_of_enemies_and_bosses(
    mut enemy_query: Query<(
        &Speed,
        &EnemySpawnPoint,
        &mut PathCheckpointNumber,
        &mut Transform,
    )>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    for (speed, spawn_point, mut path_checkpoint_number, mut transform) in enemy_query.iter_mut() {
        let window = window_query
            .get_single()
            .expect("Primary window should exist");

        let (x_scale_next_checkpoint, y_scale_next_checkpoint) =
            get_x_y_scale_of_checkpoint(spawn_point, path_checkpoint_number.number + 1, &window);
        let next_checkpoint_vec = Vec3::new(
            window.width() * x_scale_next_checkpoint,
            window.height() * y_scale_next_checkpoint,
            0.0,
        );
        let turn_instruction =
            get_current_turn_instruction(spawn_point, path_checkpoint_number.number);
        let translation = speed.speed
            * time.delta_seconds()
            * get_translation_from_turn_instruction(&turn_instruction);
        let new_position = transform.translation + translation;
        // Check if next path checkpoint is reached
        if match turn_instruction {
            // Who does this?
            Down => new_position.y < next_checkpoint_vec.y,
            Up => new_position.y > next_checkpoint_vec.y,
            Left => new_position.x < next_checkpoint_vec.x,
            Right => new_position.x > next_checkpoint_vec.x,
        } {
            // next path checkpoint is reached
            let distance_to_go =
                translation.length() - transform.translation.distance(next_checkpoint_vec);
            path_checkpoint_number.number += 1;
            let turn_instruction =
                get_current_turn_instruction(spawn_point, path_checkpoint_number.number);
            transform.translation +=
                distance_to_go * get_translation_from_turn_instruction(&turn_instruction);
        } else {
            // next path checkpoint is not yet reached
            transform.translation = new_position;
        }
    }
}

/// Translates the spawn point enum to a transform
pub fn generate_spawn_point_transform_from_enum(
    enemy_spawn_point_enum: EnemySpawnPoint,
    window: &Window,
) -> Transform {
    let (x_scale, y_scale) = get_x_y_scale_of_checkpoint(&enemy_spawn_point_enum, 0, window);
    Transform::from_xyz(window.width() * x_scale, window.height() * y_scale, 0.0)
}

/// Returns bool with whether sprite needs to be flipped from spawnpoint depending if spawn point is towards left or right of screen
pub fn check_if_sprite_needs_to_be_flipped_from_spawnpoint(spawn_point: EnemySpawnPoint) -> bool {
    match spawn_point {
        EnemySpawnPoint::TopLeft => true,
        EnemySpawnPoint::TopRight => false,
        EnemySpawnPoint::Left => true,
        EnemySpawnPoint::Right => false,
        EnemySpawnPoint::BottomLeft => true,
        EnemySpawnPoint::BottomRight => false,
    }
}

/// Returns a translation vector with the direction to go in based on the given turn instruction
fn get_translation_from_turn_instruction(turn_instruction: &TurnInstruction) -> Vec3 {
    use TurnInstruction::*;
    match turn_instruction {
        Down => Vec3::new(0.0, -1.0, 0.0),
        Up => Vec3::new(0.0, 1.0, 0.0),
        Left => Vec3::new(-1.0, 0.0, 0.0),
        Right => Vec3::new(1.0, 0.0, 0.0),
    }
}

/// Returns the xy scale of checkpoints given the spawn point and checkpoint number an enemy has reached
fn get_x_y_scale_of_checkpoint(
    spawn_point: &EnemySpawnPoint,
    check_point_number: usize,
    window: &Window,
) -> (f32, f32) {
    let (x_scale, y_scale) = *match spawn_point {
        EnemySpawnPoint::TopLeft => TOP_LEFT_ROUTE_CHECKPOINTS
            .get(check_point_number)
            .expect("Turning Point should exist"),
        EnemySpawnPoint::TopRight => TOP_RIGHT_ROUTE_CHECKPOINTS
            .get(check_point_number)
            .expect("Turning Point should exist"),
        EnemySpawnPoint::Right => RIGHT_ROUTE_CHECKPOINTS
            .get(check_point_number)
            .expect("Turning Point should exist"),
        EnemySpawnPoint::Left => LEFT_ROUTE_CHECKPOINTS
            .get(check_point_number)
            .expect("Turning Point should exist"),
        EnemySpawnPoint::BottomLeft => BOTTOM_LEFT_ROUTE_CHECKPOINTS
            .get(check_point_number)
            .expect("Turning Point should exist"),
        EnemySpawnPoint::BottomRight => BOTTOM_RIGHT_ROUTE_CHECKPOINTS
            .get(check_point_number)
            .expect("Turning Point should exist"),
    };
    (
        x_scale * (WINDOW_WIDTH_IN_DEV / window.width()),
        y_scale * (WINDOW_HEIGHT_IN_DEV / window.height()),
    )
}

/// Returns the current turn instruction based on the spawn point and current check point number of an enemy
pub fn get_current_turn_instruction(
    spawn_point: &EnemySpawnPoint,
    check_point_number: usize,
) -> &TurnInstruction {
    match spawn_point {
        EnemySpawnPoint::TopLeft => TOP_LEFT_ROUTE_TURN_INSTRUCTIONS
            .get(check_point_number)
            .expect("Turn instruction should exist"),
        EnemySpawnPoint::TopRight => TOP_RIGHT_ROUTE_TURN_INSTRUCTIONS
            .get(check_point_number)
            .expect("Turn instruction should exist"),
        EnemySpawnPoint::Left => LEFT_ROUTE_TURN_INSTRUCTIONS
            .get(check_point_number)
            .expect("Turn instruction should exist"),
        EnemySpawnPoint::Right => RIGHT_ROUTE_TURN_INSTRUCTIONS
            .get(check_point_number)
            .expect("Turn instruction should exist"),
        EnemySpawnPoint::BottomLeft => BOTTOM_LEFT_ROUTE_TURN_INSTRUCTIONS
            .get(check_point_number)
            .expect("Turn instruction should exist"),
        EnemySpawnPoint::BottomRight => BOTTOM_RIGHT_ROUTE_TURN_INSTRUCTIONS
            .get(check_point_number)
            .expect("Turn instruction should exist"),
    }
}

/// System for despawning enemies when they are out of screen
pub fn despawn_enemy_if_out_of_screen(
    mut commands: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut number_of_enemies_unlived_current_round: ResMut<NumberOfEnemiesUnlivedThisRound>,
    mut streak_indicator: ResMut<StreakIndicator>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().expect("Window should exist");
    for (enemy_entity, enemy_transform) in &enemy_query {
        if enemy_transform.translation.x > window.width() * 0.7
            || enemy_transform.translation.x < -window.width() * 0.7
            || enemy_transform.translation.y > window.height() * 0.7
            || enemy_transform.translation.y < -window.height() * 0.7
        {
            // Despawn enemy and set resources accordingly
            commands.entity(enemy_entity).despawn_recursive();
            number_of_enemies_unlived_current_round.number += 1;
            streak_indicator.number = 0;
        }
    }
}

/// System for tracking the collision of enemies with the castle
pub fn enemy_collision_with_castle(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    castle_query: Query<&Transform, With<castle::components::Castle>>,
    mut number_of_enemies_unlived_current_round: ResMut<NumberOfEnemiesUnlivedThisRound>,
    mut streak_indicator: ResMut<StreakIndicator>,
    mut number_of_lives_left: ResMut<castle::resources::NumberOfLivesLeft>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut enemies_being_typed: ResMut<EnemiesBeingTyped>,
) {
    if let Ok(_) = castle_query.get_single() {
        for (entity, transform) in enemy_query.iter_mut() {
            if transform.translation.y > -80.0
                && transform.translation.y < 125.0
                && transform.translation.x > -150.0
                && transform.translation.x < 150.0
            {
                // Enemy hit castle. Check where collision happened
                let explosion_transform_option = if transform.translation.y.abs() < 15.0
                    && transform.translation.x < 0.0
                {
                    Some(Vec3::new(-150.0, 0.0, 10.0))
                } else if transform.translation.y.abs() < 15.0 && transform.translation.x > 0.0 {
                    Some(Vec3::new(150.0, 0.0, 10.0))
                } else if transform.translation.x.abs() < 15.0 && transform.translation.y < 0.0 {
                    Some(Vec3::new(0.0, -80.0, 10.0))
                } else if transform.translation.x.abs() < 15.0 && transform.translation.y > 0.0 {
                    Some(Vec3::new(0.0, 125.0, 10.0))
                } else {
                    None
                };

                if let Some(explosion_translation) = explosion_transform_option {
                    let explosion_transform = Transform::from_translation(explosion_translation);
                    // Spawn explosion/death animation
                    let texture_handle: Handle<Image> =
                        asset_server.load("sprites/effects/explosion.png");
                    let texture_atlas =
                        TextureAtlasLayout::from_grid(Vec2::new(192.0, 192.0), 9, 1, None, None);
                    let texture_atlas_handle: Handle<TextureAtlasLayout> =
                        texture_atlases.add(texture_atlas);

                    let explosion_animation: ExplosionAnimation = ExplosionAnimation {
                        length_of_animation: 9,
                        animation_timer: Timer::from_seconds(
                            effects::components::EXPLOSION_ANIMATION_SPEED,
                            TimerMode::Repeating,
                        ),
                    };

                    commands.spawn((
                        SpriteSheetBundle {
                            transform: explosion_transform,
                            atlas: TextureAtlas {
                                layout: texture_atlas_handle,
                                index: 0,
                                ..default()
                            },
                            texture: texture_handle,
                            ..default()
                        },
                        Explosion {},
                        explosion_animation,
                    ));
                }
                // Despawn enemy and set resources accordingly
                commands.entity(entity).despawn_recursive();
                number_of_enemies_unlived_current_round.number += 1;
                streak_indicator.number = 0;
                if enemies_being_typed.vec_of_enemies.contains(&entity) {
                    enemies_being_typed.vec_of_enemies.retain(|&x| x != entity);
                    if enemies_being_typed.vec_of_enemies.len() == 0 {
                        enemies_being_typed.indicator = false;
                    }
                }
                if let Some(val) = number_of_lives_left.number.checked_sub(1) {
                    number_of_lives_left.number = val;
                }
            }
        }
    }
}

/// Despawn the enemies on restart event
pub fn despawn_enemies_on_restart(
    mut commands: Commands,
    enemy_query: Query<Entity, With<Enemy>>,
    mut restart_event_reader: EventReader<Restart>,
) {
    for _ in restart_event_reader.read() {
        for enemy_entity in enemy_query.iter() {
            commands.entity(enemy_entity).despawn_recursive();
        }
    }
}
