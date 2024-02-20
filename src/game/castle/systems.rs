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
