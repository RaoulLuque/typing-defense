use bevy::window::PrimaryWindow;
use rand::Rng;

use super::*;

pub fn spawn_castle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let castle_number = i32::to_string(&rand::thread_rng().gen_range(0..4));
    let castle_sprite_path = format!("sprites/castle/castle{}.png", castle_number);
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, window.height() * 0.05, 1.0),
            texture: asset_server.load(castle_sprite_path),
            ..default()
        },
        Castle {},
        Name::new("Castle"),
    ));
}

pub fn despawn_castle_if_all_lives_are_gone_and_spawn_destroyed_castle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut castle_query: Query<(Entity, &mut Handle<Image>), With<Castle>>,
    number_of_lives_left: Res<NumberOfLivesLeft>,
) {
    if number_of_lives_left.number == 0 {
        if let Ok((castle_entity, mut castle_image)) = castle_query.get_single_mut() {
            commands.entity(castle_entity).remove::<Castle>();
            commands.entity(castle_entity).insert(DestroyedCastle {});
            *castle_image = asset_server.load("sprites/castle/castleDestroyed.png");
        }
    }
}
