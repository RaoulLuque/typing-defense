use bevy::window::PrimaryWindow;

use super::*;

const TREE_ANIMATION_SPEED: f32 = 0.1;
const TREE_SPAWN_POINTS_SCALES: [(f32, f32); 8] = [
    (0.40, 0.10),
    (0.35, -0.10),
    (-0.25, -0.13),
    (-0.42, 0.2),
    (0.05, 0.4),
    (-0.05, -0.4),
    (0.35, 0.4),
    (0.27, -0.2),
];

pub fn spawn_trees(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query
        .get_single()
        .expect("Primary window should exist");

    for (x_scale, y_scale) in TREE_SPAWN_POINTS_SCALES {
        let tree_spawnpoint = generate_spawn_point_transform_from_x_y(x_scale, y_scale, &window);

        let texture_handle: Handle<Image> = asset_server.load("sprites/decorations/tree.png");
        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(192.0, 192.0), 4, 1, None, None);
        let texture_atlas_handle: Handle<TextureAtlas> = texture_atlases.add(texture_atlas);

        let tree_wiggle_animation: TreeWiggleAnimation = TreeWiggleAnimation {
            length_of_animation: 4,
            animation_timer: Timer::from_seconds(TREE_ANIMATION_SPEED, TimerMode::Repeating),
        };

        commands.spawn((
            SpriteSheetBundle {
                transform: tree_spawnpoint,
                sprite: TextureAtlasSprite {
                    index: 0,
                    ..default()
                },
                texture_atlas: texture_atlas_handle,
                ..default()
            },
            tree_wiggle_animation,
        ));
    }
}

pub fn animate_trees(
    time: Res<Time>,
    mut tree_query: Query<(&mut TreeWiggleAnimation, &mut TextureAtlasSprite)>,
) {
    for (mut tree_animation, mut atlas_sprite) in &mut tree_query {
        tree_animation.animation_timer.tick(time.delta());
        if tree_animation.animation_timer.just_finished() {
            atlas_sprite.index = if atlas_sprite.index == tree_animation.length_of_animation - 1 {
                0
            } else {
                atlas_sprite.index + 1
            };
        }
    }
}

fn generate_spawn_point_transform_from_x_y(
    x_scale: f32,
    y_scale: f32,
    window: &Window,
) -> Transform {
    Transform::from_xyz(window.width() * x_scale, window.height() * y_scale, 1.0)
}
