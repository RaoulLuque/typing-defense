use super::*;

pub fn spawn_castle(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::default(),
            texture: asset_server.load("sprites/castle.png"),
            ..default()
        },
        Castle {},
    ));
}

pub fn despawn_castle_if_all_lives_are_gone(
    mut commands: Commands,
    castle_query: Query<Entity, With<Castle>>,
    number_of_lives_left: Res<NumberOfLivesLeft>,
) {
    if number_of_lives_left.number == 0 {
        if let Ok(castle_entity) = castle_query.get_single() {
            commands.entity(castle_entity).despawn_recursive();
        }
    }
}
