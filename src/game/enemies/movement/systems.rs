use enemies::rounds::resources::NumberOfEnemiesTypedCurrentRound;

use super::*;

// The movement is calculated relatively to the screen height/width in order to work for multiple resolutions
// One grid block on the background corresponds to:
// In width: 0.03333
// In height: 0.05263

enum TurnInstruction {
    Left,
    Right,
    Up,
    Down,
}

use bevy::window::PrimaryWindow;
use TurnInstruction::*;

// Checkpoints are given in percent * 100 of the screen width/height respectively.
// E.g.: 540 is 0.5 in height if the screen height is 1080 (Full HD)
// The last checkpoint is always far enough out of the screen such that enemies have despawned before
const TOP_LEFT_ROUTE_CHECKPOINTS: [(f32, f32); 6] = [
    (-0.31, 0.5),
    (-0.31, 0.31578),
    (0.0, 0.31778),
    (0.0, -0.31178),
    (0.20899, -0.31178),
    (0.20899, -1.0),
];
const TOP_LEFT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 6] =
    [Down, Right, Down, Right, Down, Down];

const BOTTOM_RIGHT_ROUTE_CHECKPOINTS: [(f32, f32); 6] = [
    (0.20899, -0.5),
    (0.20899, -0.31178),
    (0.0, -0.31178),
    (0.0, 0.31778),
    (-0.31, 0.31578),
    (-0.31, 1.0),
];
const BOTTOM_RIGHT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 6] = [Up, Left, Up, Left, Up, Up];

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
const TOP_RIGHT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 9] =
    [Down, Left, Up, Left, Down, Left, Down, Left, Left];

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
const BOTTOM_LEFT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 9] =
    [Right, Up, Right, Up, Right, Down, Right, Up, Up];

const LEFT_ROUTE_CHECKPOINTS: [(f32, f32); 2] = [(-0.5, 0.01), (1.0, 0.01)];
const LEFT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 2] = [Right, Right];

const RIGHT_ROUTE_CHECKPOINTS: [(f32, f32); 2] = [(0.5, 0.01), (-1.0, 0.01)];
const RIGHT_ROUTE_TURN_INSTRUCTIONS: [TurnInstruction; 2] = [Left, Left];

pub fn update_position_of_enemies(
    mut enemy_query: Query<
        (
            &Speed,
            &EnemySpawnPoint,
            &mut PathCheckpointNumber,
            &mut Transform,
        ),
        With<Enemy>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    for (speed, spawn_point, mut path_checkpoint_number, mut transform) in enemy_query.iter_mut() {
        let window = window_query
            .get_single()
            .expect("Primary window should exist");

        let (x_scale_next_checkpoint, y_scale_next_checkpoint) =
            get_x_y_scale_of_checkpoint(spawn_point, path_checkpoint_number.number + 1);
        let next_checkpoint_vec = Vec3::new(
            window.width() * x_scale_next_checkpoint,
            window.height() * y_scale_next_checkpoint,
            0.0,
        );
        let turn_instruction = get_turn_instruction(spawn_point, path_checkpoint_number.number);
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
            let turn_instruction = get_turn_instruction(spawn_point, path_checkpoint_number.number);
            transform.translation +=
                distance_to_go * get_translation_from_turn_instruction(&turn_instruction);
        } else {
            // next path checkpoint is not yet reached
            transform.translation = new_position;
        }
    }
}

pub fn generate_spawn_point_transform_from_enum(
    enemy_spawn_point_enum: EnemySpawnPoint,
    window: &Window,
) -> Transform {
    let (x_scale, y_scale) = match enemy_spawn_point_enum {
        EnemySpawnPoint::TopLeft => TOP_LEFT_ROUTE_CHECKPOINTS
            .get(0)
            .expect("Array shouldn't be empty"),
        EnemySpawnPoint::TopRight => TOP_RIGHT_ROUTE_CHECKPOINTS
            .get(0)
            .expect("Array shouldn't be empty"),
        EnemySpawnPoint::Right => RIGHT_ROUTE_CHECKPOINTS
            .get(0)
            .expect("Array shouldn't be empty"),
        EnemySpawnPoint::Left => LEFT_ROUTE_CHECKPOINTS
            .get(0)
            .expect("Array shouldn't be empty"),
        EnemySpawnPoint::BottomLeft => BOTTOM_LEFT_ROUTE_CHECKPOINTS
            .get(0)
            .expect("Array shouldn't be empty"),
        EnemySpawnPoint::BottomRight => BOTTOM_RIGHT_ROUTE_CHECKPOINTS
            .get(0)
            .expect("Array shouldn't be empty"),
    };
    Transform::from_xyz(window.width() * x_scale, window.height() * y_scale, 0.0)
}

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

fn get_translation_from_turn_instruction(turn_instruction: &TurnInstruction) -> Vec3 {
    use TurnInstruction::*;
    match turn_instruction {
        Down => Vec3::new(0.0, -1.0, 0.0),
        Up => Vec3::new(0.0, 1.0, 0.0),
        Left => Vec3::new(-1.0, 0.0, 0.0),
        Right => Vec3::new(1.0, 0.0, 0.0),
    }
}

fn get_x_y_scale_of_checkpoint(
    spawn_point: &EnemySpawnPoint,
    check_point_number: usize,
) -> (f32, f32) {
    *match spawn_point {
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
    }
}

fn get_turn_instruction(
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